package circuit

import (
	"testing"

	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/test"
)

func TestTemplateCircuit(t *testing.T) {
	// Clear any existing circuit definition

	// Set up a simple circuit definition: private[0] + private[1] = public[0]
	// With witness: private = [3, 5], public = [8]
	testJSON := `{
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

	// Create a template circuit from JSON
	circuit, err := NewTemplateCircuitFromJSON(testJSON)
	if err != nil {
		t.Fatalf("Failed to create template circuit: %v", err)
	}

	// Verify circuit structure
	if len(circuit.PublicVariables) != 1 {
		t.Errorf("Expected 1 public variable, got %d", len(circuit.PublicVariables))
	}
	if len(circuit.PrivateVariables) != 2 {
		t.Errorf("Expected 2 private variables, got %d", len(circuit.PrivateVariables))
	}

	// Create witness circuit with actual values
	witnessCircuit := &TemplateCircuit{
		PublicVariables:  []frontend.Variable{8},    // public[0] = 8
		PrivateVariables: []frontend.Variable{3, 5}, // private[0] = 3, private[1] = 5
	}

	// Test circuit compilation with gnark
	assert := test.NewAssert(t)

	// This should compile and be satisfied: 3 + 5 = 8
	assert.CheckCircuit(circuit, test.WithValidAssignment(witnessCircuit))
}

func TestTemplateCircuitWithInvalidWitness(t *testing.T) {
	// Clear any existing circuit definition

	// Set up the same circuit definition: private[0] + private[1] = public[0]
	testJSON := `{
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

	// Create a template circuit from JSON
	circuit, err := NewTemplateCircuitFromJSON(testJSON)
	if err != nil {
		t.Fatalf("Failed to create template circuit: %v", err)
	}

	// Create witness circuit with INVALID values: 3 + 5 ≠ 7
	invalidWitnessCircuit := &TemplateCircuit{
		PublicVariables:  []frontend.Variable{7},    // public[0] = 7 (WRONG!)
		PrivateVariables: []frontend.Variable{3, 5}, // private[0] = 3, private[1] = 5
	}

	// Test circuit compilation with gnark - this should FAIL
	assert := test.NewAssert(t)

	// We expect this to fail because 3 + 5 = 8, not 7
	// Use WithInvalidAssignment to indicate we expect the constraint to fail
	assert.CheckCircuit(circuit, test.WithInvalidAssignment(invalidWitnessCircuit))
}

func TestNewTemplateCircuitWithoutDefinition(t *testing.T) {
	// Try to create circuit with nil definition
	_, err := NewTemplateCircuit(nil)
	if err == nil {
		t.Error("Expected error when creating circuit with nil definition")
	}
}

func TestTemplateCircuitDefineWithoutDefinition(t *testing.T) {
	// Clear circuit definition

	// Create a circuit (this should fail)
	circuit := &TemplateCircuit{
		PublicVariables:  make([]frontend.Variable, 1),
		PrivateVariables: make([]frontend.Variable, 1),
	}

	// Try to define without global definition
	err := circuit.Define(nil) // We pass nil since we expect it to fail before using the API
	if err == nil {
		t.Error("Expected error when defining circuit without global definition")
	}
}

func TestSimpleArithmeticCircuit(t *testing.T) {
	// Clear any existing circuit definition

	// Create a simple arithmetic circuit: a + b = c
	// With witness: private = [3, 5], public = [8]
	simpleJSON := `{
  "private_len": 2,
  "public_len": 1,
  "local_len": 0,
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
          "t": "public",
          "v": 0
        }
      ]
    }
  ]
}`

	// Create a template circuit from JSON
	circuit, err := NewTemplateCircuitFromJSON(simpleJSON)
	if err != nil {
		t.Fatalf("Failed to create simple template circuit: %v", err)
	}

	// Verify circuit structure
	if len(circuit.PublicVariables) != 1 {
		t.Errorf("Expected 1 public variable, got %d", len(circuit.PublicVariables))
	}
	if len(circuit.PrivateVariables) != 2 {
		t.Errorf("Expected 2 private variables, got %d", len(circuit.PrivateVariables))
	}

	// Create witness circuit with actual values
	witnessCircuit := &TemplateCircuit{
		PublicVariables:  []frontend.Variable{8},    // public[0] = 8
		PrivateVariables: []frontend.Variable{3, 5}, // private[0] = 3, private[1] = 5
	}

	// Test compilation and satisfaction
	assert := test.NewAssert(t)
	assert.CheckCircuit(circuit, test.WithValidAssignment(witnessCircuit))
}

func TestConstantVariableCircuit(t *testing.T) {
	// Clear any existing circuit definition

	// Create a circuit with constant: private_var + 5 = public_var
	// With witness: private = [3], public = [8] (because 3 + 5 = 8)
	constantJSON := `{
  "private_len": 1,
  "public_len": 1,
  "local_len": 0,
  "operations": [
    {
      "op": "add",
      "in": [
        {
          "t": "private",
          "v": 0
        },
        {
          "t": "constant",
          "v": "5"
        }
      ],
      "out": [
        {
          "t": "public",
          "v": 0
        }
      ]
    }
  ]
}`

	// Create a template circuit from JSON
	circuit, err := NewTemplateCircuitFromJSON(constantJSON)
	if err != nil {
		t.Fatalf("Failed to create constant template circuit: %v", err)
	}

	// Create witness circuit with actual values
	witnessCircuit := &TemplateCircuit{
		PublicVariables:  []frontend.Variable{8}, // public[0] = 8
		PrivateVariables: []frontend.Variable{3}, // private[0] = 3 (3 + 5 = 8)
	}

	// Test compilation and satisfaction
	assert := test.NewAssert(t)
	assert.CheckCircuit(circuit, test.WithValidAssignment(witnessCircuit))
}
