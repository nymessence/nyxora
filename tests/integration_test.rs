// tests/integration_test.rs

#[cfg(test)]
mod integration_tests {
    use std::process::Command;

    #[test]
    fn test_wallet_generate_command() {
        // Test that the wallet can generate a new wallet
        let output = Command::new("cargo")
            .args(&["run", "--bin", "nyxora-wallet", "--", "generate"])
            .current_dir("nyxora-wallet")
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
            .current_dir("nyxora-wallet")
            .output()
            .expect("Failed to execute generate command");
        
        // Then check the address
        let output = Command::new("cargo")
            .args(&["run", "--bin", "nyxora-wallet", "--", "address"])
            .current_dir("nyxora-wallet")
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