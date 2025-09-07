package main

import (
	"testing"
)

func TestMultiInputOperations(t *testing.T) {
	// Clear any existing circuit definition
	clearCircuitDefine()

	// Create a circuit with multi-input add operation: a + b + c = result
	multiAddJSON := `{
  "private_len": 3,
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
        },
        {
          "t": "private",
          "v": 2
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

	// Set the circuit definition
	err := setCircuitDefine(multiAddJSON)
	if err != nil {
		t.Fatalf("Failed to set multi-input circuit definition: %v", err)
	}

	// Create a template circuit
	circuit, err := NewTemplateCircuit()
	if err != nil {
		t.Fatalf("Failed to create multi-input template circuit: %v", err)
	}

	// Verify circuit structure
	if len(circuit.PublicVariables) != 1 {
		t.Errorf("Expected 1 public variable, got %d", len(circuit.PublicVariables))
	}
	if len(circuit.PrivateVariables) != 3 {
		t.Errorf("Expected 3 private variables, got %d", len(circuit.PrivateVariables))
	}

	t.Logf("Multi-input circuit created successfully")
}

func TestMultiInputMultiplication(t *testing.T) {
	// Clear any existing circuit definition
	clearCircuitDefine()

	// Create a circuit with multi-input multiplication: a * b * c = result
	multiMulJSON := `{
  "private_len": 3,
  "public_len": 1,
  "local_len": 0,
  "operations": [
    {
      "op": "mul",
      "in": [
        {
          "t": "private",
          "v": 0
        },
        {
          "t": "private",
          "v": 1
        },
        {
          "t": "private",
          "v": 2
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

	// Set the circuit definition
	err := setCircuitDefine(multiMulJSON)
	if err != nil {
		t.Fatalf("Failed to set multi-multiplication circuit definition: %v", err)
	}

	// Create a template circuit
	circuit, err := NewTemplateCircuit()
	if err != nil {
		t.Fatalf("Failed to create multi-multiplication template circuit: %v", err)
	}

	// Verify circuit structure
	if len(circuit.PublicVariables) != 1 {
		t.Errorf("Expected 1 public variable, got %d", len(circuit.PublicVariables))
	}
	if len(circuit.PrivateVariables) != 3 {
		t.Errorf("Expected 3 private variables, got %d", len(circuit.PrivateVariables))
	}

	t.Logf("Multi-multiplication circuit created successfully")
}

func TestToBinaryOperation(t *testing.T) {
	// Clear any existing circuit definition
	clearCircuitDefine()

	// Create a circuit with ToBinary operation: convert number to 8 bits
	toBinaryJSON := `{
  "private_len": 1,
  "public_len": 0,
  "local_len": 8,
  "operations": [
    {
      "op": "to_binary",
      "in": [
        {
          "t": "private",
          "v": 0
        }
      ],
      "out": [
        {
          "t": "local",
          "v": 0
        },
        {
          "t": "local",
          "v": 1
        },
        {
          "t": "local",
          "v": 2
        },
        {
          "t": "local",
          "v": 3
        },
        {
          "t": "local",
          "v": 4
        },
        {
          "t": "local",
          "v": 5
        },
        {
          "t": "local",
          "v": 6
        },
        {
          "t": "local",
          "v": 7
        }
      ]
    }
  ]
}`

	// Set the circuit definition
	err := setCircuitDefine(toBinaryJSON)
	if err != nil {
		t.Fatalf("Failed to set to_binary circuit definition: %v", err)
	}

	// Create a template circuit
	circuit, err := NewTemplateCircuit()
	if err != nil {
		t.Fatalf("Failed to create to_binary template circuit: %v", err)
	}

	// Verify circuit structure
	if len(circuit.PublicVariables) != 0 {
		t.Errorf("Expected 0 public variables, got %d", len(circuit.PublicVariables))
	}
	if len(circuit.PrivateVariables) != 1 {
		t.Errorf("Expected 1 private variable, got %d", len(circuit.PrivateVariables))
	}

	t.Logf("ToBinary circuit created successfully")
}

func TestFromBinaryOperation(t *testing.T) {
	// Clear any existing circuit definition
	clearCircuitDefine()

	// Create a circuit with FromBinary operation: convert 4 bits to number
	fromBinaryJSON := `{
  "private_len": 4,
  "public_len": 1,
  "local_len": 0,
  "operations": [
    {
      "op": "from_binary",
      "in": [
        {
          "t": "private",
          "v": 0
        },
        {
          "t": "private",
          "v": 1
        },
        {
          "t": "private",
          "v": 2
        },
        {
          "t": "private",
          "v": 3
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

	// Set the circuit definition
	err := setCircuitDefine(fromBinaryJSON)
	if err != nil {
		t.Fatalf("Failed to set from_binary circuit definition: %v", err)
	}

	// Create a template circuit
	circuit, err := NewTemplateCircuit()
	if err != nil {
		t.Fatalf("Failed to create from_binary template circuit: %v", err)
	}

	// Verify circuit structure
	if len(circuit.PublicVariables) != 1 {
		t.Errorf("Expected 1 public variable, got %d", len(circuit.PublicVariables))
	}
	if len(circuit.PrivateVariables) != 4 {
		t.Errorf("Expected 4 private variables, got %d", len(circuit.PrivateVariables))
	}

	t.Logf("FromBinary circuit created successfully")
}

func TestMulAccOperation(t *testing.T) {
	// Clear any existing circuit definition
	clearCircuitDefine()

	// Create a circuit with MulAcc operation: acc + a * b = result
	mulAccJSON := `{
  "private_len": 3,
  "public_len": 1,
  "local_len": 0,
  "operations": [
    {
      "op": "mul_acc",
      "in": [
        {
          "t": "private",
          "v": 0
        },
        {
          "t": "private",
          "v": 1
        },
        {
          "t": "private",
          "v": 2
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

	// Set the circuit definition
	err := setCircuitDefine(mulAccJSON)
	if err != nil {
		t.Fatalf("Failed to set mul_acc circuit definition: %v", err)
	}

	// Create a template circuit
	circuit, err := NewTemplateCircuit()
	if err != nil {
		t.Fatalf("Failed to create mul_acc template circuit: %v", err)
	}

	// Verify circuit structure
	if len(circuit.PublicVariables) != 1 {
		t.Errorf("Expected 1 public variable, got %d", len(circuit.PublicVariables))
	}
	if len(circuit.PrivateVariables) != 3 {
		t.Errorf("Expected 3 private variables, got %d", len(circuit.PrivateVariables))
	}

	t.Logf("MulAcc circuit created successfully")
}
