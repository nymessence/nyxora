# tests/quantum_tests.py
import unittest
import sys
import os

# Add the quantum directories to the path
sys.path.append(os.path.join(os.path.dirname(__file__), '..', 'quantum', 'circuits'))
sys.path.append(os.path.join(os.path.dirname(__file__), '..', 'quantum', 'verifier'))

from hex_hadamard_cnot import HexagonalQuantumCircuit, generate_quantum_proof
from verifier import QuantumProofVerifier


class TestHexagonalQuantumCircuit(unittest.TestCase):
    def test_circuit_initialization(self):
        """Test that the hexagonal quantum circuit initializes correctly."""
        circuit = HexagonalQuantumCircuit(layers=2, qubits_per_side=1)
        
        # Check that the qubit count is calculated correctly
        expected_qubit_count = 1 + 3 * (2 - 1) * 2  # 1 + 3*1*2 = 7
        self.assertEqual(circuit.qubit_count, expected_qubit_count)
        
        # Check that the circuit has the correct number of qubits
        self.assertEqual(circuit.circuit.num_qubits, expected_qubit_count)
    
    def test_circuit_descriptor(self):
        """Test that the circuit descriptor is generated correctly."""
        circuit = HexagonalQuantumCircuit(layers=2, qubits_per_side=1)
        descriptor = circuit.get_circuit_descriptor()
        
        # Check that the descriptor is a valid JSON string
        import json
        parsed = json.loads(descriptor)
        
        # Check that required fields are present
        self.assertIn('qubit_count', parsed)
        self.assertIn('layers', parsed)
        self.assertIn('circuit_width', parsed)
        self.assertIn('circuit_depth', parsed)
        self.assertIn('gate_counts', parsed)
        
        # Check that values are reasonable
        self.assertEqual(parsed['qubit_count'], circuit.qubit_count)
        self.assertEqual(parsed['layers'], circuit.layers)
    
    def test_circuit_simulation(self):
        """Test that the circuit can be simulated."""
        circuit = HexagonalQuantumCircuit(layers=2, qubits_per_side=1)
        circuit.build_circuit()
        
        result = circuit.simulate()
        
        # Check that the result contains required fields
        self.assertIn('measurement_results', result)
        self.assertIn('proof_artifact', result)
        self.assertIn('circuit_descriptor', result)
        self.assertIn('counts', result)
        
        # Check that measurement results match qubit count
        self.assertEqual(len(result['measurement_results']), circuit.qubit_count)
        
        # Check that all measurement results are 0 or 1
        for bit in result['measurement_results']:
            self.assertIn(bit, [0, 1])
    
    def test_generate_quantum_proof(self):
        """Test that quantum proofs can be generated."""
        proof = generate_quantum_proof(qubit_count=8, difficulty_level=1)

        # Check that the proof contains required fields
        self.assertIn('circuit_descriptor', proof)
        self.assertIn('measurement_results', proof)
        self.assertIn('proof_artifact', proof)
        self.assertIn('qubit_count', proof)
        self.assertIn('timestamp', proof)
        self.assertIn('difficulty_level', proof)

        # Check that values are reasonable
        self.assertGreater(proof['qubit_count'], 0)  # Should have at least some qubits
        self.assertGreater(len(proof['proof_artifact']), 0)
        self.assertEqual(len(proof['measurement_results']), proof['qubit_count'])


class TestQuantumProofVerifier(unittest.TestCase):
    def test_proof_verification(self):
        """Test that valid proofs are verified correctly."""
        # Generate a proof
        proof = generate_quantum_proof(qubit_count=6, difficulty_level=1)
        
        # Create a verifier and verify the proof
        verifier = QuantumProofVerifier()
        is_valid = verifier.verify_proof(proof)
        
        self.assertTrue(is_valid)
    
    def test_invalid_proof_rejection(self):
        """Test that invalid proofs are rejected."""
        # Generate a proof
        proof = generate_quantum_proof(qubit_count=6, difficulty_level=1)
        
        # Modify the proof to make it invalid
        invalid_proof = proof.copy()
        invalid_proof['measurement_results'][0] = 1 - invalid_proof['measurement_results'][0]  # Flip first bit
        
        # Create a verifier and try to verify the invalid proof
        verifier = QuantumProofVerifier()
        is_valid = verifier.verify_proof(invalid_proof)
        
        self.assertFalse(is_valid)
    
    def test_replay_attack_prevention(self):
        """Test that the same proof cannot be verified twice."""
        # Generate a proof
        proof = generate_quantum_proof(qubit_count=6, difficulty_level=1)
        
        # Create a verifier and verify the proof
        verifier = QuantumProofVerifier()
        is_valid1 = verifier.verify_proof(proof)
        
        # Try to verify the same proof again
        is_valid2 = verifier.verify_proof(proof)
        
        # First verification should succeed, second should fail
        self.assertTrue(is_valid1)
        self.assertFalse(is_valid2)
    
    def test_missing_fields(self):
        """Test that proofs with missing fields are rejected."""
        # Create a proof with missing fields
        incomplete_proof = {
            'circuit_descriptor': '{"qubit_count": 5}',
            'measurement_results': [0, 1, 0, 1, 0]
            # Missing proof_artifact, qubit_count, timestamp
        }
        
        # Create a verifier and try to verify the incomplete proof
        verifier = QuantumProofVerifier()
        is_valid = verifier.verify_proof(incomplete_proof)
        
        self.assertFalse(is_valid)


class TestQuantumSimulator(unittest.TestCase):
    def test_simulation_verification(self):
        """Test that simulated proofs can be verified."""
        # Import the simulator
        sys.path.append(os.path.join(os.path.dirname(__file__), '..', 'quantum', 'simulator'))
        from simulator import QuantumSimulator
        
        # Create a simulator and generate a simulated proof
        simulator = QuantumSimulator(noise_level=0.001)
        proof = simulator.simulate_quantum_proof(qubit_count=6, difficulty_level=1)
        
        # Verify the simulated proof
        is_valid = simulator.verify_simulation(proof)
        
        self.assertTrue(is_valid)
    
    def test_invalid_simulation_rejection(self):
        """Test that invalid simulated proofs are rejected."""
        # Import the simulator
        sys.path.append(os.path.join(os.path.dirname(__file__), '..', 'quantum', 'simulator'))
        from simulator import QuantumSimulator
        
        # Create a simulator
        simulator = QuantumSimulator(noise_level=0.001)
        
        # Create an invalid proof
        invalid_proof = {
            'circuit_descriptor': '{"qubit_count": 5, "layers": 2, "circuit_width": 10, "circuit_depth": 5}',
            'measurement_results': [0, 1, 0, 1],  # Wrong length
            'proof_artifact': 'invalid_hash',
            'qubit_count': 5,
            'timestamp': __import__('time').time(),
            'difficulty_level': 1
        }
        
        # Try to verify the invalid proof
        is_valid = simulator.verify_simulation(invalid_proof)
        
        self.assertFalse(is_valid)


if __name__ == '__main__':
    unittest.main()