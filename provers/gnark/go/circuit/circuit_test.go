package circuit

import (
	"testing"
)

// Test JSON data from the subcircuit_test.rs example (same as in types_test.go)
const testCircuitJSONForCircuit = `{
  "private_len": 4,
  "public_len": 3,
  "local_len": 3,
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

func TestNewTemplateCircuitFromJSON(t *testing.T) {
	// Create circuit from JSON
	circuitDef, err := NewTemplateCircuitFromJSON(testCircuitJSONForCircuit)
	if err != nil {
		t.Fatalf("Failed to create circuit from JSON: %v", err)
	}

	// Verify circuit was created
	if circuitDef == nil {
		t.Error("Expected circuit to be created")
	}

	// Verify the circuit has the expected number of variables
	if len(circuitDef.PublicVariables) != 3 {
		t.Errorf("Expected 3 public variables, got %d", len(circuitDef.PublicVariables))
	}
	if len(circuitDef.PrivateVariables) != 4 {
		t.Errorf("Expected 4 private variables, got %d", len(circuitDef.PrivateVariables))
	}
}

func TestNewTemplateCircuitFromInvalidJSON(t *testing.T) {
	// Try to create circuit from invalid JSON
	invalidJSON := `{"invalid": json}`
	circuit, err := NewTemplateCircuitFromJSON(invalidJSON)
	if err == nil {
		t.Error("Expected error when creating circuit from invalid JSON")
	}

	// Circuit should not be created
	if circuit != nil {
		t.Error("Expected no circuit to be created from invalid JSON")
	}
}

func TestNewTemplateCircuitWithNilDefinition(t *testing.T) {
	// Try to create circuit with nil definition
	circuit, err := NewTemplateCircuit(nil)
	if err == nil {
		t.Error("Expected error when creating circuit with nil definition")
	}

	// Circuit should not be created
	if circuit != nil {
		t.Error("Expected no circuit to be created with nil definition")
	}
}

func TestConcurrentCircuitCreation(t *testing.T) {
	// Test concurrent circuit creation - each circuit is independent
	done := make(chan bool, 2)
	var circuit1, circuit2 *TemplateCircuit
	var err1, err2 error

	// Goroutine 1: Create circuit repeatedly
	go func() {
		for i := 0; i < 100; i++ {
			circuit1, err1 = NewTemplateCircuitFromJSON(testCircuitJSONForCircuit)
			if err1 != nil {
				t.Errorf("Failed to create circuit in goroutine 1: %v", err1)
			}
		}
		done <- true
	}()

	// Goroutine 2: Create circuit repeatedly
	go func() {
		for i := 0; i < 100; i++ {
			circuit2, err2 = NewTemplateCircuitFromJSON(testCircuitJSONForCircuit)
			if err2 != nil {
				t.Errorf("Failed to create circuit in goroutine 2: %v", err2)
			}
		}
		done <- true
	}()

	// Wait for both goroutines to complete
	<-done
	<-done

	// Verify both circuits were created successfully
	if circuit1 == nil {
		t.Error("Expected circuit1 to be created")
	}
	if circuit2 == nil {
		t.Error("Expected circuit2 to be created")
	}
}
