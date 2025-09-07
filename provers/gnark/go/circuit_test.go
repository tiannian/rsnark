package main

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

func TestSetCircuitDefine(t *testing.T) {
	// Clear any existing circuit definition
	clearCircuitDefine()

	// Initially, no circuit should be set
	if isCircuitDefineSet() {
		t.Error("Expected no circuit definition to be set initially")
	}

	// Set circuit definition
	err := setCircuitDefine(testCircuitJSONForCircuit)
	if err != nil {
		t.Fatalf("Failed to set circuit definition: %v", err)
	}

	// Now circuit should be set
	if !isCircuitDefineSet() {
		t.Error("Expected circuit definition to be set")
	}

	// Verify circuit is set (we can't verify content without getCircuitDefine)
	if !isCircuitDefineSet() {
		t.Fatal("Expected circuit definition to be set after successful setCircuitDefine")
	}
}

func TestSetCircuitDefineInvalidJSON(t *testing.T) {
	// Clear any existing circuit definition
	clearCircuitDefine()

	// Try to set invalid JSON
	invalidJSON := `{"invalid": json}`
	err := setCircuitDefine(invalidJSON)
	if err == nil {
		t.Error("Expected error when setting invalid JSON")
	}

	// Circuit should still not be set
	if isCircuitDefineSet() {
		t.Error("Expected no circuit definition to be set after invalid JSON")
	}
}

func TestClearCircuitDefine(t *testing.T) {
	// Set circuit definition
	err := setCircuitDefine(testCircuitJSONForCircuit)
	if err != nil {
		t.Fatalf("Failed to set circuit definition: %v", err)
	}

	// Verify it's set
	if !isCircuitDefineSet() {
		t.Error("Expected circuit definition to be set")
	}

	// Clear it
	clearCircuitDefine()

	// Verify it's cleared
	if isCircuitDefineSet() {
		t.Error("Expected circuit definition to be cleared")
	}

	// Circuit should not be set after clearing
	if isCircuitDefineSet() {
		t.Error("Expected circuit definition to not be set after clearing")
	}
}

func TestConcurrentAccess(t *testing.T) {
	// Clear any existing circuit definition
	clearCircuitDefine()

	// Test concurrent access to ensure thread safety
	done := make(chan bool, 2)

	// Goroutine 1: Set circuit definition repeatedly
	go func() {
		for i := 0; i < 100; i++ {
			setCircuitDefine(testCircuitJSONForCircuit)
		}
		done <- true
	}()

	// Goroutine 2: Read circuit definition repeatedly
	go func() {
		for i := 0; i < 100; i++ {
			isCircuitDefineSet()
		}
		done <- true
	}()

	// Wait for both goroutines to complete
	<-done
	<-done

	// Verify final state
	if !isCircuitDefineSet() {
		t.Error("Expected circuit definition to be set after concurrent access")
	}
}
