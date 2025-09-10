package circuit

import (
	"encoding/json"
	"math/big"
	"strings"
	"testing"
)

// Test JSON data from the subcircuit_test.rs example
const testCircuitJSON = `{
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
    },
    {
      "op": "mul",
      "in": [
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
          "t": "local",
          "v": 1
        }
      ]
    },
    {
      "op": "assert_is_equal",
      "in": [
        {
          "t": "local",
          "v": 1
        },
        {
          "t": "public",
          "v": 1
        }
      ],
      "out": []
    },
    {
      "op": "add",
      "in": [
        {
          "t": "public",
          "v": 0
        },
        {
          "t": "public",
          "v": 1
        }
      ],
      "out": [
        {
          "t": "local",
          "v": 2
        }
      ]
    },
    {
      "op": "assert_is_equal",
      "in": [
        {
          "t": "local",
          "v": 2
        },
        {
          "t": "public",
          "v": 2
        }
      ],
      "out": []
    }
  ]
}`

func TestParseCircuitDefinition(t *testing.T) {
	cd, err := ParseCircuitDefinition([]byte(testCircuitJSON))
	if err != nil {
		t.Fatalf("Failed to parse circuit definition: %v", err)
	}

	// Test basic fields
	if cd.PrivateLen != 4 {
		t.Errorf("Expected PrivateLen=4, got %d", cd.PrivateLen)
	}
	if cd.PublicLen != 3 {
		t.Errorf("Expected PublicLen=3, got %d", cd.PublicLen)
	}
	if cd.LocalLen != 3 {
		t.Errorf("Expected LocalLen=3, got %d", cd.LocalLen)
	}
	if len(cd.Operations) != 6 {
		t.Errorf("Expected 6 operations, got %d", len(cd.Operations))
	}

	// Test first operation (add)
	op0 := cd.Operations[0]
	if op0.Op != OpAdd {
		t.Errorf("Expected first operation to be 'add', got '%s'", op0.Op)
	}
	if len(op0.Inputs) != 2 {
		t.Errorf("Expected 2 inputs for first operation, got %d", len(op0.Inputs))
	}
	if len(op0.Outputs) != 1 {
		t.Errorf("Expected 1 output for first operation, got %d", len(op0.Outputs))
	}

	// Test variable types
	input0 := op0.Inputs[0]
	if input0.Type != string(VarPrivate) {
		t.Errorf("Expected first input to be private, got %s", input0.Type)
	}
	if input0.ValueInt == nil || *input0.ValueInt != 0 {
		t.Errorf("Expected first input to be Private(0), got %s", input0.String())
	}

	output0 := op0.Outputs[0]
	if output0.Type != string(VarLocal) {
		t.Errorf("Expected first output to be local, got %s", output0.Type)
	}
	if output0.ValueInt == nil || *output0.ValueInt != 0 {
		t.Errorf("Expected first output to be Local(0), got %s", output0.String())
	}
}

func TestVariableTypeFields(t *testing.T) {
	// Test Public variable
	idx5 := uint64(5)
	pubVar := VariableType{Type: string(VarPublic), ValueInt: &idx5}
	if pubVar.ValueInt == nil || *pubVar.ValueInt != 5 {
		t.Errorf("Expected Public(5), got %s", pubVar.String())
	}
	if pubVar.ValueBigInt != nil {
		t.Errorf("Public variable should not have ValueBigInt set")
	}

	// Test Private variable
	idx3 := uint64(3)
	privVar := VariableType{Type: string(VarPrivate), ValueInt: &idx3}
	if privVar.ValueInt == nil || *privVar.ValueInt != 3 {
		t.Errorf("Expected Private(3), got %s", privVar.String())
	}
	if privVar.ValueBigInt != nil {
		t.Errorf("Private variable should not have ValueBigInt set")
	}

	// Test Local variable
	idx1 := uint64(1)
	localVar := VariableType{Type: string(VarLocal), ValueInt: &idx1}
	if localVar.ValueInt == nil || *localVar.ValueInt != 1 {
		t.Errorf("Expected Local(1), got %s", localVar.String())
	}
	if localVar.ValueBigInt != nil {
		t.Errorf("Local variable should not have ValueBigInt set")
	}

	// Test Constant variable
	constValue := big.NewInt(123456789)
	constVar := VariableType{Type: string(VarConstant), ValueBigInt: constValue}
	expected := big.NewInt(123456789)
	if constVar.ValueBigInt == nil || constVar.ValueBigInt.Cmp(expected) != 0 {
		t.Errorf("Expected Constant(123456789), got %s", constVar.String())
	}
	if constVar.ValueInt != nil {
		t.Errorf("Constant variable should not have ValueInt set")
	}

	// Test Constant variable (large number)
	largeValue := new(big.Int)
	largeValue.SetString("12345678901234567890123456789", 10)
	constVar2 := VariableType{Type: string(VarConstant), ValueBigInt: largeValue}
	if constVar2.ValueBigInt == nil || constVar2.ValueBigInt.Cmp(largeValue) != 0 {
		t.Errorf("Expected Constant(%s), got %s", largeValue.String(), constVar2.String())
	}
	if constVar2.ValueInt != nil {
		t.Errorf("Constant variable should not have ValueInt set")
	}
}

func TestCircuitDefinitionString(t *testing.T) {
	cd, err := ParseCircuitDefinition([]byte(testCircuitJSON))
	if err != nil {
		t.Fatalf("Failed to parse circuit definition: %v", err)
	}

	str := cd.String()
	expected := "CircuitDefinition{private_len: 4, public_len: 3, local_len: 3, operations: 6}"
	if str != expected {
		t.Errorf("Expected: %s\nGot: %s", expected, str)
	}
}

func TestOperationString(t *testing.T) {
	cd, err := ParseCircuitDefinition([]byte(testCircuitJSON))
	if err != nil {
		t.Fatalf("Failed to parse circuit definition: %v", err)
	}

	op := cd.Operations[0]
	str := op.String()
	if !strings.Contains(str, "add") {
		t.Errorf("Operation string should contain 'add', got: %s", str)
	}
	if !strings.Contains(str, "Private(0)") {
		t.Errorf("Operation string should contain 'Private(0)', got: %s", str)
	}
	if !strings.Contains(str, "Private(1)") {
		t.Errorf("Operation string should contain 'Private(1)', got: %s", str)
	}
	if !strings.Contains(str, "Local(0)") {
		t.Errorf("Operation string should contain 'Local(0)', got: %s", str)
	}
}

func TestCircuitDefinitionParsing(t *testing.T) {
	// Parse from JSON
	cd, err := ParseCircuitDefinition([]byte(testCircuitJSON))
	if err != nil {
		t.Fatalf("Failed to parse circuit definition: %v", err)
	}

	// Test that all variables are properly parsed into the correct fields
	for i, op := range cd.Operations {
		for j, input := range op.Inputs {
			switch input.Type {
			case string(VarPublic), string(VarPrivate), string(VarLocal):
				if input.ValueInt == nil {
					t.Errorf("Operation %d, input %d: %s variable should have ValueInt set", i, j, input.Type)
				}
				if input.ValueBigInt != nil {
					t.Errorf("Operation %d, input %d: %s variable should not have ValueBigInt set", i, j, input.Type)
				}
			case string(VarConstant):
				if input.ValueBigInt == nil {
					t.Errorf("Operation %d, input %d: constant variable should have ValueBigInt set", i, j)
				}
				if input.ValueInt != nil {
					t.Errorf("Operation %d, input %d: constant variable should not have ValueInt set", i, j)
				}
			}
		}

		for j, output := range op.Outputs {
			switch output.Type {
			case string(VarPublic), string(VarPrivate), string(VarLocal):
				if output.ValueInt == nil {
					t.Errorf("Operation %d, output %d: %s variable should have ValueInt set", i, j, output.Type)
				}
				if output.ValueBigInt != nil {
					t.Errorf("Operation %d, output %d: %s variable should not have ValueBigInt set", i, j, output.Type)
				}
			case string(VarConstant):
				if output.ValueBigInt == nil {
					t.Errorf("Operation %d, output %d: constant variable should have ValueBigInt set", i, j)
				}
				if output.ValueInt != nil {
					t.Errorf("Operation %d, output %d: constant variable should not have ValueInt set", i, j)
				}
			}
		}
	}
}

// TestWitnessTypes has been removed - witness functionality moved to types package

func TestAllOpCodes(t *testing.T) {
	// Test that all defined OpCodes can be marshaled/unmarshaled
	opCodes := []OpCode{
		OpAdd, OpMulAcc, OpNeg, OpSub, OpMul, OpDivUnchecked, OpDiv,
		OpInverse, OpToBinary, OpFromBinary, OpXor, OpOr, OpAnd, OpSelect,
		OpLookup2, OpIsZero, OpCmp, OpAssertIsEqual, OpAssertIsDifferent,
		OpAssertIsBoolean, OpAssertIsCrumb, OpAssertIsLessOrEqual, OpPrintln,
	}

	for _, op := range opCodes {
		operation := Operation{
			Op:      op,
			Inputs:  []VariableType{},
			Outputs: []VariableType{},
		}

		jsonBytes, err := json.Marshal(operation)
		if err != nil {
			t.Errorf("Failed to marshal operation with opcode %s: %v", op, err)
		}

		var operation2 Operation
		err = json.Unmarshal(jsonBytes, &operation2)
		if err != nil {
			t.Errorf("Failed to unmarshal operation with opcode %s: %v", op, err)
		}

		if operation.Op != operation2.Op {
			t.Errorf("OpCode mismatch: %s != %s", operation.Op, operation2.Op)
		}
	}
}
