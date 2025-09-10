package prover

import (
	"math/big"
	"os"
	"path/filepath"
	"testing"

	"github.com/tiannian/rsnark/provers-gnark/circuit"
	"github.com/tiannian/rsnark/provers-gnark/prover/types"
)

// Test circuit JSON: a simple arithmetic circuit (private[0] + private[1] = public[0])
const testCircuitJSON = `{
  "private_len": 2,
  "public_len": 1,
  "local_len": 1,
  "operations": [
    {
      "op": "add",
      "in": [
        {
          "t": "private",
          "v": 0
        },
        {
          "t": "private",
          "v": 1
        }
      ],
      "out": [
        {
          "t": "local",
          "v": 0
        }
      ]
    },
    {
      "op": "assert_is_equal",
      "in": [
        {
          "t": "local",
          "v": 0
        },
        {
          "t": "public",
          "v": 0
        }
      ],
      "out": []
    }
  ]
}`

func TestGroth16ProverCreation(t *testing.T) {
	// Test creating a new Groth16 prover
	prover := NewGroth16Prover(types.CurveBN254)
	if prover == nil {
		t.Error("Expected prover to be created")
		return
	}

	if prover.curve != types.CurveBN254 {
		t.Errorf("Expected curve to be BN254, got %v", prover.curve)
	}
}

func TestGroth16CompileFromDefinition(t *testing.T) {
	prover := NewGroth16Prover(types.CurveBN254)

	// Create circuit definition from JSON
	circuitDef, err := circuit.ParseCircuitDefinition([]byte(testCircuitJSON))
	if err != nil {
		t.Fatalf("Failed to create circuit definition: %v", err)
	}

	// Test compilation
	compiled, err := prover.CompileFromDefinition(types.CurveBN254, circuitDef)
	if err != nil {
		t.Fatalf("Failed to compile circuit: %v", err)
	}

	if compiled == nil {
		t.Error("Expected compiled circuit to be created")
		return
	}

	if compiled.CS == nil {
		t.Error("Expected constraint system to be set")
	}
}

func TestGroth16Setup(t *testing.T) {
	prover := NewGroth16Prover(types.CurveBN254)

	// Create and compile circuit
	circuitDef, err := circuit.ParseCircuitDefinition([]byte(testCircuitJSON))
	if err != nil {
		t.Fatalf("Failed to create circuit definition: %v", err)
	}

	compiled, err := prover.CompileFromDefinition(types.CurveBN254, circuitDef)
	if err != nil {
		t.Fatalf("Failed to compile circuit: %v", err)
	}

	// Test setup
	pk, vk, err := prover.Setup(compiled)
	if err != nil {
		t.Fatalf("Failed to setup Groth16: %v", err)
	}

	if pk == nil {
		t.Error("Expected proving key to be created")
	}

	if vk == nil {
		t.Error("Expected verifying key to be created")
	}
}

func TestGroth16ProveAndVerify(t *testing.T) {
	prover := NewGroth16Prover(types.CurveBN254)

	// Create and compile circuit
	circuitDef, err := circuit.ParseCircuitDefinition([]byte(testCircuitJSON))
	if err != nil {
		t.Fatalf("Failed to create circuit definition: %v", err)
	}

	compiled, err := prover.CompileFromDefinition(types.CurveBN254, circuitDef)
	if err != nil {
		t.Fatalf("Failed to compile circuit: %v", err)
	}

	// Setup
	pk, vk, err := prover.Setup(compiled)
	if err != nil {
		t.Fatalf("Failed to setup Groth16: %v", err)
	}

	// Create witness: private = [3, 5], public = [8] (3 + 5 = 8)
	witness := types.NewTemplateWitness(
		[]*big.Int{big.NewInt(8)},                // public variables
		[]*big.Int{big.NewInt(3), big.NewInt(5)}, // private variables
	)

	// Test proving
	proofBytes, err := prover.Prove(compiled, pk, witness)
	if err != nil {
		t.Fatalf("Failed to generate proof: %v", err)
	}

	if len(proofBytes) == 0 {
		t.Error("Expected proof bytes to be generated")
	}

	// Create public witness for verification
	publicWitness := types.NewTemplatePublicWitnessFromTemplate(witness)

	// Test verification
	err = prover.Verify(proofBytes, types.CurveBN254, vk, publicWitness)
	if err != nil {
		t.Fatalf("Failed to verify proof: %v", err)
	}
}

func TestGroth16ProveAndVerifyInvalidWitness(t *testing.T) {
	prover := NewGroth16Prover(types.CurveBN254)

	// Create and compile circuit
	circuitDef, err := circuit.ParseCircuitDefinition([]byte(testCircuitJSON))
	if err != nil {
		t.Fatalf("Failed to create circuit definition: %v", err)
	}

	compiled, err := prover.CompileFromDefinition(types.CurveBN254, circuitDef)
	if err != nil {
		t.Fatalf("Failed to compile circuit: %v", err)
	}

	// Setup
	pk, _, err := prover.Setup(compiled)
	if err != nil {
		t.Fatalf("Failed to setup Groth16: %v", err)
	}

	// Create INVALID witness: private = [3, 5], public = [7] (3 + 5 ≠ 7)
	invalidWitness := types.NewTemplateWitness(
		[]*big.Int{big.NewInt(7)},                // public variables (WRONG!)
		[]*big.Int{big.NewInt(3), big.NewInt(5)}, // private variables
	)

	// Test proving - this should fail because the constraint is not satisfied
	_, err = prover.Prove(compiled, pk, invalidWitness)
	if err == nil {
		t.Error("Expected error when proving with invalid witness")
	}
}

func TestGroth16VerifyWithWrongPublicWitness(t *testing.T) {
	prover := NewGroth16Prover(types.CurveBN254)

	// Create and compile circuit
	circuitDef, err := circuit.ParseCircuitDefinition([]byte(testCircuitJSON))
	if err != nil {
		t.Fatalf("Failed to create circuit definition: %v", err)
	}

	compiled, err := prover.CompileFromDefinition(types.CurveBN254, circuitDef)
	if err != nil {
		t.Fatalf("Failed to compile circuit: %v", err)
	}

	// Setup
	pk, vk, err := prover.Setup(compiled)
	if err != nil {
		t.Fatalf("Failed to setup Groth16: %v", err)
	}

	// Create valid witness: private = [3, 5], public = [8]
	validWitness := types.NewTemplateWitness(
		[]*big.Int{big.NewInt(8)},                // public variables
		[]*big.Int{big.NewInt(3), big.NewInt(5)}, // private variables
	)

	// Generate valid proof
	proofBytes, err := prover.Prove(compiled, pk, validWitness)
	if err != nil {
		t.Fatalf("Failed to generate proof: %v", err)
	}

	// Create WRONG public witness for verification
	wrongPublicWitness := types.NewTemplatePublicWitness(
		[]*big.Int{big.NewInt(7)}, // wrong public value
	)

	// Test verification - this should fail
	err = prover.Verify(proofBytes, types.CurveBN254, vk, wrongPublicWitness)
	if err == nil {
		t.Error("Expected error when verifying with wrong public witness")
	}
}

func TestGroth16KeySerialization(t *testing.T) {
	prover := NewGroth16Prover(types.CurveBN254)

	// Create and compile circuit
	circuitDef, err := circuit.ParseCircuitDefinition([]byte(testCircuitJSON))
	if err != nil {
		t.Fatalf("Failed to create circuit definition: %v", err)
	}

	compiled, err := prover.CompileFromDefinition(types.CurveBN254, circuitDef)
	if err != nil {
		t.Fatalf("Failed to compile circuit: %v", err)
	}

	// Setup
	pk, vk, err := prover.Setup(compiled)
	if err != nil {
		t.Fatalf("Failed to setup Groth16: %v", err)
	}

	// Test proving key serialization
	pkBytes, err := pk.Serialize()
	if err != nil {
		t.Fatalf("Failed to serialize proving key: %v", err)
	}

	if len(pkBytes) == 0 {
		t.Error("Expected proving key bytes to be generated")
	}

	// Test proving key deserialization
	newPk := types.NewGroth16ProvingKey()
	err = newPk.Deserialize(pkBytes, types.CurveBN254)
	if err != nil {
		t.Fatalf("Failed to deserialize proving key: %v", err)
	}

	// Test verifying key serialization
	vkBytes, err := vk.Serialize()
	if err != nil {
		t.Fatalf("Failed to serialize verifying key: %v", err)
	}

	if len(vkBytes) == 0 {
		t.Error("Expected verifying key bytes to be generated")
	}

	// Test verifying key deserialization
	newVk := types.NewGroth16VerifyingKey()
	err = newVk.Deserialize(vkBytes, types.CurveBN254)
	if err != nil {
		t.Fatalf("Failed to deserialize verifying key: %v", err)
	}
}

func TestGroth16KeyFileOperations(t *testing.T) {
	prover := NewGroth16Prover(types.CurveBN254)

	// Create and compile circuit
	circuitDef, err := circuit.ParseCircuitDefinition([]byte(testCircuitJSON))
	if err != nil {
		t.Fatalf("Failed to create circuit definition: %v", err)
	}

	compiled, err := prover.CompileFromDefinition(types.CurveBN254, circuitDef)
	if err != nil {
		t.Fatalf("Failed to compile circuit: %v", err)
	}

	// Setup
	pk, vk, err := prover.Setup(compiled)
	if err != nil {
		t.Fatalf("Failed to setup Groth16: %v", err)
	}

	// Create temporary directory for test files
	tempDir, err := os.MkdirTemp("", "groth16_test")
	if err != nil {
		t.Fatalf("Failed to create temp directory: %v", err)
	}
	defer os.RemoveAll(tempDir)

	// Test proving key file operations
	pkFile := filepath.Join(tempDir, "proving_key.bin")
	err = pk.SaveToFile(pkFile)
	if err != nil {
		t.Fatalf("Failed to save proving key to file: %v", err)
	}

	newPk := types.NewGroth16ProvingKey()
	err = newPk.LoadFromFile(pkFile, types.CurveBN254)
	if err != nil {
		t.Fatalf("Failed to load proving key from file: %v", err)
	}

	// Test verifying key file operations
	vkFile := filepath.Join(tempDir, "verifying_key.bin")
	err = vk.SaveToFile(vkFile)
	if err != nil {
		t.Fatalf("Failed to save verifying key to file: %v", err)
	}

	newVk := types.NewGroth16VerifyingKey()
	err = newVk.LoadFromFile(vkFile, types.CurveBN254)
	if err != nil {
		t.Fatalf("Failed to load verifying key from file: %v", err)
	}
}

func TestGroth16CompiledCircuitSerialization(t *testing.T) {
	prover := NewGroth16Prover(types.CurveBN254)

	// Create and compile circuit
	circuitDef, err := circuit.ParseCircuitDefinition([]byte(testCircuitJSON))
	if err != nil {
		t.Fatalf("Failed to create circuit definition: %v", err)
	}

	compiled, err := prover.CompileFromDefinition(types.CurveBN254, circuitDef)
	if err != nil {
		t.Fatalf("Failed to compile circuit: %v", err)
	}

	// Test compiled circuit serialization
	circuitBytes, err := compiled.Serialize()
	if err != nil {
		t.Fatalf("Failed to serialize compiled circuit: %v", err)
	}

	if len(circuitBytes) == 0 {
		t.Error("Expected compiled circuit bytes to be generated")
	}

	// Test compiled circuit deserialization
	newCompiled := types.NewCompiledCircuit()
	err = newCompiled.Deserialize(circuitBytes, types.CurveBN254)
	if err != nil {
		t.Fatalf("Failed to deserialize compiled circuit: %v", err)
	}

	// Test compiled circuit file operations
	tempDir, err := os.MkdirTemp("", "groth16_circuit_test")
	if err != nil {
		t.Fatalf("Failed to create temp directory: %v", err)
	}
	defer os.RemoveAll(tempDir)

	circuitFile := filepath.Join(tempDir, "compiled_circuit.bin")
	err = compiled.SaveToFile(circuitFile)
	if err != nil {
		t.Fatalf("Failed to save compiled circuit to file: %v", err)
	}

	loadedCompiled := types.NewCompiledCircuit()
	err = loadedCompiled.LoadFromFile(circuitFile, types.CurveBN254)
	if err != nil {
		t.Fatalf("Failed to load compiled circuit from file: %v", err)
	}
}

func TestGroth16MultipleProofs(t *testing.T) {
	prover := NewGroth16Prover(types.CurveBN254)

	// Create and compile circuit
	circuitDef, err := circuit.ParseCircuitDefinition([]byte(testCircuitJSON))
	if err != nil {
		t.Fatalf("Failed to create circuit definition: %v", err)
	}

	compiled, err := prover.CompileFromDefinition(types.CurveBN254, circuitDef)
	if err != nil {
		t.Fatalf("Failed to compile circuit: %v", err)
	}

	// Setup
	pk, vk, err := prover.Setup(compiled)
	if err != nil {
		t.Fatalf("Failed to setup Groth16: %v", err)
	}

	// Test multiple proofs with different witnesses
	testCases := []struct {
		name    string
		private []*big.Int
		public  []*big.Int
	}{
		{"Case 1: 3+5=8", []*big.Int{big.NewInt(3), big.NewInt(5)}, []*big.Int{big.NewInt(8)}},
		{"Case 2: 10+20=30", []*big.Int{big.NewInt(10), big.NewInt(20)}, []*big.Int{big.NewInt(30)}},
		{"Case 3: 0+0=0", []*big.Int{big.NewInt(0), big.NewInt(0)}, []*big.Int{big.NewInt(0)}},
		{"Case 4: 1+1=2", []*big.Int{big.NewInt(1), big.NewInt(1)}, []*big.Int{big.NewInt(2)}},
	}

	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			// Create witness
			witness := types.NewTemplateWitness(tc.public, tc.private)

			// Generate proof
			proofBytes, err := prover.Prove(compiled, pk, witness)
			if err != nil {
				t.Fatalf("Failed to generate proof for %s: %v", tc.name, err)
			}

			// Verify proof
			publicWitness := types.NewTemplatePublicWitnessFromTemplate(witness)
			err = prover.Verify(proofBytes, types.CurveBN254, vk, publicWitness)
			if err != nil {
				t.Fatalf("Failed to verify proof for %s: %v", tc.name, err)
			}
		})
	}
}
