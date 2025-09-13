package prover

import (
	"math/big"
	"testing"

	"github.com/tiannian/rsnark/provers-gnark/circuit"
	"github.com/tiannian/rsnark/provers-gnark/prover/types"
)

// Test circuit JSON: a simple arithmetic circuit (private[0] + private[1] = public[0])
// Same circuit as Groth16 test to ensure consistency
const testPlonkCircuitJSON = `{
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

func TestPlonkProverCreation(t *testing.T) {
	// Test creating a new PLONK prover
	prover := NewPlonkProver(types.CurveBN254)
	if prover == nil {
		t.Error("Expected prover to be created")
		return
	}

	if prover.curve != types.CurveBN254 {
		t.Errorf("Expected curve to be BN254, got %v", prover.curve)
	}

	if prover.CurveId() != types.CurveBN254 {
		t.Errorf("Expected CurveId() to return BN254, got %v", prover.CurveId())
	}
}

func TestPlonkCompileFromDefinition(t *testing.T) {
	prover := NewPlonkProver(types.CurveBN254)

	// Create circuit definition from JSON
	circuitDef, err := circuit.ParseCircuitDefinition([]byte(testPlonkCircuitJSON))
	if err != nil {
		t.Fatalf("Failed to create circuit definition: %v", err)
	}

	// Test compilation
	compiled, err := prover.Compile(circuitDef)
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

func TestPlonkSetup(t *testing.T) {
	prover := NewPlonkProver(types.CurveBN254)

	// Create and compile circuit
	circuitDef, err := circuit.ParseCircuitDefinition([]byte(testPlonkCircuitJSON))
	if err != nil {
		t.Fatalf("Failed to create circuit definition: %v", err)
	}

	compiled, err := prover.Compile(circuitDef)
	if err != nil {
		t.Fatalf("Failed to compile circuit: %v", err)
	}

	// Test setup
	pk, vk, err := prover.Setup(compiled)
	if err != nil {
		t.Fatalf("Failed to setup PLONK: %v", err)
	}

	if pk == nil {
		t.Error("Expected proving key to be created")
	}

	if vk == nil {
		t.Error("Expected verifying key to be created")
	}
}

func TestPlonkProveAndVerify(t *testing.T) {
	prover := NewPlonkProver(types.CurveBN254)

	// Create and compile circuit
	circuitDef, err := circuit.ParseCircuitDefinition([]byte(testPlonkCircuitJSON))
	if err != nil {
		t.Fatalf("Failed to create circuit definition: %v", err)
	}

	compiled, err := prover.Compile(circuitDef)
	if err != nil {
		t.Fatalf("Failed to compile circuit: %v", err)
	}

	// Setup
	pk, vk, err := prover.Setup(compiled)
	if err != nil {
		t.Fatalf("Failed to setup PLONK: %v", err)
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

	// Create public witness for verification
	publicWitness := types.NewTemplatePublicWitnessFromTemplate(witness)

	// Test verification
	err = prover.Verify(proofBytes, vk, publicWitness)
	if err != nil {
		t.Fatalf("Failed to verify proof: %v", err)
	}
}

func TestPlonkProveAndVerifyInvalidWitness(t *testing.T) {
	prover := NewPlonkProver(types.CurveBN254)

	// Create and compile circuit
	circuitDef, err := circuit.ParseCircuitDefinition([]byte(testPlonkCircuitJSON))
	if err != nil {
		t.Fatalf("Failed to create circuit definition: %v", err)
	}

	compiled, err := prover.Compile(circuitDef)
	if err != nil {
		t.Fatalf("Failed to compile circuit: %v", err)
	}

	// Setup
	pk, _, err := prover.Setup(compiled)
	if err != nil {
		t.Fatalf("Failed to setup PLONK: %v", err)
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

func TestPlonkVerifyWithWrongPublicWitness(t *testing.T) {
	prover := NewPlonkProver(types.CurveBN254)

	// Create and compile circuit
	circuitDef, err := circuit.ParseCircuitDefinition([]byte(testPlonkCircuitJSON))
	if err != nil {
		t.Fatalf("Failed to create circuit definition: %v", err)
	}

	compiled, err := prover.Compile(circuitDef)
	if err != nil {
		t.Fatalf("Failed to compile circuit: %v", err)
	}

	// Setup
	pk, vk, err := prover.Setup(compiled)
	if err != nil {
		t.Fatalf("Failed to setup PLONK: %v", err)
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
	err = prover.Verify(proofBytes, vk, wrongPublicWitness)
	if err == nil {
		t.Error("Expected error when verifying with wrong public witness")
	}
}

func TestPlonkKeySerialization(t *testing.T) {
	prover := NewPlonkProver(types.CurveBN254)

	// Create and compile circuit
	circuitDef, err := circuit.ParseCircuitDefinition([]byte(testPlonkCircuitJSON))
	if err != nil {
		t.Fatalf("Failed to create circuit definition: %v", err)
	}

	compiled, err := prover.Compile(circuitDef)
	if err != nil {
		t.Fatalf("Failed to compile circuit: %v", err)
	}

	// Setup
	pk, vk, err := prover.Setup(compiled)
	if err != nil {
		t.Fatalf("Failed to setup PLONK: %v", err)
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
	newPk := types.NewPlonkProvingKey()
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
	newVk := types.NewPlonkVerifyingKey()
	err = newVk.Deserialize(vkBytes, types.CurveBN254)
	if err != nil {
		t.Fatalf("Failed to deserialize verifying key: %v", err)
	}
}

func TestPlonkKeyFileOperations(t *testing.T) {
	prover := NewPlonkProver(types.CurveBN254)

	// Create and compile circuit
	circuitDef, err := circuit.ParseCircuitDefinition([]byte(testPlonkCircuitJSON))
	if err != nil {
		t.Fatalf("Failed to create circuit definition: %v", err)
	}

	compiled, err := prover.Compile(circuitDef)
	if err != nil {
		t.Fatalf("Failed to compile circuit: %v", err)
	}

	// Setup
	pk, vk, err := prover.Setup(compiled)
	if err != nil {
		t.Fatalf("Failed to setup PLONK: %v", err)
	}

	// Test proving key serialization
	_, err = pk.Serialize()
	if err != nil {
		t.Fatalf("Failed to serialize proving key: %v", err)
	}

	// Test verifying key serialization
	_, err = vk.Serialize()
	if err != nil {
		t.Fatalf("Failed to serialize verifying key: %v", err)
	}
}

func TestPlonkCompiledCircuitSerialization(t *testing.T) {
	prover := NewPlonkProver(types.CurveBN254)

	// Create and compile circuit
	circuitDef, err := circuit.ParseCircuitDefinition([]byte(testPlonkCircuitJSON))
	if err != nil {
		t.Fatalf("Failed to create circuit definition: %v", err)
	}

	compiled, err := prover.Compile(circuitDef)
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
	newCompiled := types.NewPlonkCompiledCircuit()
	err = newCompiled.Deserialize(circuitBytes, types.CurveBN254)
	if err != nil {
		t.Fatalf("Failed to deserialize compiled circuit: %v", err)
	}
}

func TestPlonkMultipleProofs(t *testing.T) {
	prover := NewPlonkProver(types.CurveBN254)

	// Create and compile circuit
	circuitDef, err := circuit.ParseCircuitDefinition([]byte(testPlonkCircuitJSON))
	if err != nil {
		t.Fatalf("Failed to create circuit definition: %v", err)
	}

	compiled, err := prover.Compile(circuitDef)
	if err != nil {
		t.Fatalf("Failed to compile circuit: %v", err)
	}

	// Setup
	pk, vk, err := prover.Setup(compiled)
	if err != nil {
		t.Fatalf("Failed to setup PLONK: %v", err)
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
		{"Case 5: 100+200=300", []*big.Int{big.NewInt(100), big.NewInt(200)}, []*big.Int{big.NewInt(300)}},
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
			err = prover.Verify(proofBytes, vk, publicWitness)
			if err != nil {
				t.Fatalf("Failed to verify proof for %s: %v", tc.name, err)
			}
		})
	}
}

func TestPlonkDifferentCurves(t *testing.T) {
	// Test different curves supported by PLONK
	curves := []struct {
		name  string
		curve types.CurveType
	}{
		{"BN254", types.CurveBN254},
		{"BLS12-381", types.CurveBLS12381},
		// Note: Other curves might have different requirements for PLONK
		// so we test them separately if needed
	}

	for _, tc := range curves {
		t.Run(tc.name, func(t *testing.T) {
			prover := NewPlonkProver(tc.curve)

			// Verify prover creation
			if prover.CurveId() != tc.curve {
				t.Errorf("Expected curve %v, got %v", tc.curve, prover.CurveId())
			}

			// Create and compile circuit (basic test)
			circuitDef, err := circuit.ParseCircuitDefinition([]byte(testPlonkCircuitJSON))
			if err != nil {
				t.Fatalf("Failed to create circuit definition: %v", err)
			}

			compiled, err := prover.Compile(circuitDef)
			if err != nil {
				t.Fatalf("Failed to compile circuit for %s: %v", tc.name, err)
			}

			if compiled == nil || compiled.CS == nil {
				t.Fatalf("Expected valid compiled circuit for %s", tc.name)
			}
		})
	}
}

func TestPlonkProverConsistency(t *testing.T) {
	// Test that multiple prover instances with same curve produce consistent results
	prover1 := NewPlonkProver(types.CurveBN254)
	prover2 := NewPlonkProver(types.CurveBN254)

	circuitDef, err := circuit.ParseCircuitDefinition([]byte(testPlonkCircuitJSON))
	if err != nil {
		t.Fatalf("Failed to create circuit definition: %v", err)
	}

	// Compile with both provers
	compiled1, err := prover1.Compile(circuitDef)
	if err != nil {
		t.Fatalf("Failed to compile with prover1: %v", err)
	}

	compiled2, err := prover2.Compile(circuitDef)
	if err != nil {
		t.Fatalf("Failed to compile with prover2: %v", err)
	}

	// Setup with both provers
	pk1, vk1, err := prover1.Setup(compiled1)
	if err != nil {
		t.Fatalf("Failed to setup with prover1: %v", err)
	}

	pk2, vk2, err := prover2.Setup(compiled2)
	if err != nil {
		t.Fatalf("Failed to setup with prover2: %v", err)
	}

	// Create same witness
	witness := types.NewTemplateWitness(
		[]*big.Int{big.NewInt(15)},               // public: 15
		[]*big.Int{big.NewInt(7), big.NewInt(8)}, // private: 7 + 8
	)

	// Generate proofs with both setups
	proof1, err := prover1.Prove(compiled1, pk1, witness)
	if err != nil {
		t.Fatalf("Failed to prove with prover1: %v", err)
	}

	proof2, err := prover2.Prove(compiled2, pk2, witness)
	if err != nil {
		t.Fatalf("Failed to prove with prover2: %v", err)
	}

	// Both proofs should be valid (though potentially different)
	publicWitness := types.NewTemplatePublicWitnessFromTemplate(witness)

	err = prover1.Verify(proof1, vk1, publicWitness)
	if err != nil {
		t.Fatalf("Failed to verify proof1 with vk1: %v", err)
	}

	err = prover2.Verify(proof2, vk2, publicWitness)
	if err != nil {
		t.Fatalf("Failed to verify proof2 with vk2: %v", err)
	}
}

// Benchmark tests for performance comparison
func BenchmarkPlonkSetup(b *testing.B) {
	prover := NewPlonkProver(types.CurveBN254)
	circuitDef, _ := circuit.ParseCircuitDefinition([]byte(testPlonkCircuitJSON))
	compiled, _ := prover.Compile(circuitDef)

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_, _, err := prover.Setup(compiled)
		if err != nil {
			b.Fatalf("Setup failed: %v", err)
		}
	}
}

func BenchmarkPlonkProve(b *testing.B) {
	prover := NewPlonkProver(types.CurveBN254)
	circuitDef, _ := circuit.ParseCircuitDefinition([]byte(testPlonkCircuitJSON))
	compiled, _ := prover.Compile(circuitDef)
	pk, _, _ := prover.Setup(compiled)

	witness := types.NewTemplateWitness(
		[]*big.Int{big.NewInt(8)},
		[]*big.Int{big.NewInt(3), big.NewInt(5)},
	)

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_, err := prover.Prove(compiled, pk, witness)
		if err != nil {
			b.Fatalf("Prove failed: %v", err)
		}
	}
}

func BenchmarkPlonkVerify(b *testing.B) {
	prover := NewPlonkProver(types.CurveBN254)
	circuitDef, _ := circuit.ParseCircuitDefinition([]byte(testPlonkCircuitJSON))
	compiled, _ := prover.Compile(circuitDef)
	pk, vk, _ := prover.Setup(compiled)

	witness := types.NewTemplateWitness(
		[]*big.Int{big.NewInt(8)},
		[]*big.Int{big.NewInt(3), big.NewInt(5)},
	)
	proof, _ := prover.Prove(compiled, pk, witness)
	publicWitness := types.NewTemplatePublicWitnessFromTemplate(witness)

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		err := prover.Verify(proof, vk, publicWitness)
		if err != nil {
			b.Fatalf("Verify failed: %v", err)
		}
	}
}
