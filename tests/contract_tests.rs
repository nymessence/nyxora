// tests/contract_tests.rs
use contracts::sdk::{ContractRuntime, ContractExecutionResult};

#[cfg(test)]
mod contract_tests {
    use super::*;

    #[test]
    fn test_deploy_contract() {
        let mut runtime = ContractRuntime::new(10000);

        let result = runtime.deploy_contract(
            "test_contract".to_string(),
            "contract code".to_string(),
            "Qvalidator123".to_string()
        );

        assert!(result.is_ok());
        assert!(runtime.contracts.contains_key("test_contract"));
    }

    #[test]
    fn test_deploy_duplicate_contract() {
        let mut runtime = ContractRuntime::new(10000);

        // Deploy first contract
        let result1 = runtime.deploy_contract(
            "test_contract".to_string(),
            "contract code".to_string(),
            "Qvalidator123".to_string()
        );
        assert!(result1.is_ok());

        // Try to deploy with same ID
        let result2 = runtime.deploy_contract(
            "test_contract".to_string(),
            "contract code".to_string(),
            "Qvalidator123".to_string()
        );

        assert!(result2.is_err());
    }

    #[test]
    fn test_quantum_randomness_contract() {
        let mut runtime = ContractRuntime::new(10000);

        // Deploy the quantum randomness contract
        let code = r#"
        contract QuantumRandomness {
            // Contract code here
        }
        "#.to_string();

        let deploy_result = runtime.deploy_contract(
            "quantum_randomness".to_string(),
            code,
            "Qvalidator123".to_string()
        );
        assert!(deploy_result.is_ok());

        // Execute requestRandomness
        let result = runtime.execute_contract(
            "quantum_randomness",
            "requestRandomness",
            vec![]
        ).unwrap();

        assert!(result.success);
        assert_eq!(result.output, "{\"requestId\": 0}");
    }

    #[test]
    fn test_quantum_nft_contract() {
        let mut runtime = ContractRuntime::new(10000);

        // Deploy the quantum NFT contract
        let code = r#"
        contract QuantumNFT {
            // Contract code here
        }
        "#.to_string();

        let deploy_result = runtime.deploy_contract(
            "quantum_nft".to_string(),
            code,
            "Qvalidator123".to_string()
        );
        assert!(deploy_result.is_ok());

        // Execute mintNFT
        let result = runtime.execute_contract(
            "quantum_nft",
            "mintNFT",
            vec!["ipfs://nft-metadata".to_string(), "0xquantumproofhash".to_string()]
        ).unwrap();

        assert!(result.success);
        // Parse the output to get the token ID
        let output: serde_json::Value = serde_json::from_str(&result.output).unwrap();
        let token_id = output["tokenId"].as_u64().unwrap();
        assert_eq!(token_id, 0);
    }

    #[test]
    fn test_invalid_contract_function() {
        let mut runtime = ContractRuntime::new(10000);

        // Deploy a contract
        let code = r#"
        contract TestContract {
            // Contract code here
        }
        "#.to_string();

        let deploy_result = runtime.deploy_contract(
            "test_contract".to_string(),
            code,
            "Qvalidator123".to_string()
        );
        assert!(deploy_result.is_ok());

        // Try to execute a non-existent function
        let result = runtime.execute_contract(
            "test_contract",
            "nonExistentFunction",
            vec![]
        ).unwrap();

        assert!(!result.success);
        assert!(result.output.contains("not found"));
    }
}

// We'll need to create a separate test for wallet functionality since it's in a different crate
#[cfg(test)]
mod integration_tests {
    use std::process::Command;

    #[test]
    fn test_wallet_generate_command() {
        // Test that the wallet can generate a new wallet
        let output = Command::new("cargo")
            .args(&["run", "--bin", "nyxora-wallet", "--", "generate"])
            .output()
            .expect("Failed to execute command");

        // Check that the command executed successfully
        assert!(output.status.success());

        // Check that the output contains expected text
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Generating new Nyxora wallet..."));
        assert!(stdout.contains("Wallet generated successfully!"));
        assert!(stdout.contains("Address: Q"));
        assert!(stdout.contains("wallet.json"));
    }

    #[test]
    fn test_wallet_address_command() {
        // First generate a wallet
        let _ = Command::new("cargo")
            .args(&["run", "--bin", "nyxora-wallet", "--", "generate"])
            .output()
            .expect("Failed to execute generate command");

        // Then check the address
        let output = Command::new("cargo")
            .args(&["run", "--bin", "nyxora-wallet", "--", "address"])
            .output()
            .expect("Failed to execute command");

        // Check that the command executed successfully
        assert!(output.status.success());

        // Check that the output is a valid address starting with Q
        let stdout = String::from_utf8_lossy(&output.stdout);
        let address = stdout.trim();
        assert!(address.starts_with('Q'));
        assert_eq!(address.len(), 40); // Q + 39 chars
    }
}