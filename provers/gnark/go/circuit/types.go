package circuit

import (
	"encoding/json"
	"fmt"
	"math/big"
	"strings"
)

// OpCode represents the operation type in the circuit
type OpCode string

const (
	OpAdd                 OpCode = "add"
	OpMulAcc              OpCode = "mul_acc"
	OpNeg                 OpCode = "neg"
	OpSub                 OpCode = "sub"
	OpMul                 OpCode = "mul"
	OpDivUnchecked        OpCode = "div_unchecked"
	OpDiv                 OpCode = "div"
	OpInverse             OpCode = "inverse"
	OpToBinary            OpCode = "to_binary"
	OpFromBinary          OpCode = "from_binary"
	OpXor                 OpCode = "xor"
	OpOr                  OpCode = "or"
	OpAnd                 OpCode = "and"
	OpSelect              OpCode = "select"
	OpLookup2             OpCode = "lookup2"
	OpIsZero              OpCode = "is_zero"
	OpCmp                 OpCode = "cmp"
	OpAssertIsEqual       OpCode = "assert_is_equal"
	OpAssertIsDifferent   OpCode = "assert_is_different"
	OpAssertIsBoolean     OpCode = "assert_is_boolean"
	OpAssertIsCrumb       OpCode = "assert_is_crumb"
	OpAssertIsLessOrEqual OpCode = "assert_is_less_or_equal"
	OpPrintln             OpCode = "println"
)

// VariableType represents a variable in the circuit with its type and value
type VariableType struct {
	Type        string   `json:"t"`
	ValueInt    *uint64  // For Public, Private, Local variables
	ValueBigInt *big.Int // For Constant variables
}

// UnmarshalJSON implements custom JSON unmarshaling based on variable type
func (vt *VariableType) UnmarshalJSON(data []byte) error {
	// First unmarshal into a temporary struct to get the type
	var temp struct {
		Type  string          `json:"t"`
		Value json.RawMessage `json:"v"`
	}

	if err := json.Unmarshal(data, &temp); err != nil {
		return fmt.Errorf("failed to unmarshal variable type: %w", err)
	}

	vt.Type = temp.Type

	// Parse value based on type
	switch vt.Type {
	case string(VarPublic), string(VarPrivate), string(VarLocal):
		var intVal uint64
		if err := json.Unmarshal(temp.Value, &intVal); err != nil {
			return fmt.Errorf("failed to parse %s variable index: %w", vt.Type, err)
		}
		vt.ValueInt = &intVal
		vt.ValueBigInt = nil

	case string(VarConstant):
		// Try string first (for hex values)
		var strVal string
		if err := json.Unmarshal(temp.Value, &strVal); err == nil {
			bigVal := new(big.Int)
			if _, ok := bigVal.SetString(strVal, 0); !ok {
				return fmt.Errorf("failed to parse constant string value: %s", strVal)
			}
			vt.ValueBigInt = bigVal
			vt.ValueInt = nil
			return nil
		}

		// Try uint64 (for small numbers)
		var intVal uint64
		if err := json.Unmarshal(temp.Value, &intVal); err == nil {
			vt.ValueBigInt = new(big.Int).SetUint64(intVal)
			vt.ValueInt = nil
			return nil
		}

		return fmt.Errorf("failed to parse constant value")

	default:
		return fmt.Errorf("unknown variable type: %s", vt.Type)
	}

	return nil
}

// VariableTypeTag represents the different types of variables
type VariableTypeTag string

const (
	VarPublic   VariableTypeTag = "public"
	VarPrivate  VariableTypeTag = "private"
	VarConstant VariableTypeTag = "constant"
	VarLocal    VariableTypeTag = "local"
)

// String returns a string representation of the variable
func (vt *VariableType) String() string {
	switch vt.Type {
	case string(VarPublic):
		if vt.ValueInt != nil {
			return fmt.Sprintf("Public(%d)", *vt.ValueInt)
		}
	case string(VarPrivate):
		if vt.ValueInt != nil {
			return fmt.Sprintf("Private(%d)", *vt.ValueInt)
		}
	case string(VarLocal):
		if vt.ValueInt != nil {
			return fmt.Sprintf("Local(%d)", *vt.ValueInt)
		}
	case string(VarConstant):
		if vt.ValueBigInt != nil {
			return fmt.Sprintf("Constant(%s)", vt.ValueBigInt.String())
		}
	}

	// Fallback for unknown or invalid types
	if vt.ValueInt != nil {
		return fmt.Sprintf("Unknown(%s: %d)", vt.Type, *vt.ValueInt)
	} else if vt.ValueBigInt != nil {
		return fmt.Sprintf("Unknown(%s: %s)", vt.Type, vt.ValueBigInt.String())
	}
	return fmt.Sprintf("Unknown(%s: <no value>)", vt.Type)
}

// Operation represents a single operation in the circuit
type Operation struct {
	Op      OpCode         `json:"op"`
	Inputs  []VariableType `json:"in"`
	Outputs []VariableType `json:"out"`
}

// String returns a string representation of the operation
func (op *Operation) String() string {
	inputStrs := make([]string, len(op.Inputs))
	for i, input := range op.Inputs {
		inputStrs[i] = input.String()
	}

	outputStrs := make([]string, len(op.Outputs))
	for i, output := range op.Outputs {
		outputStrs[i] = output.String()
	}

	return fmt.Sprintf("%s([%s]) -> [%s]", op.Op,
		strings.Join(inputStrs, ", "),
		strings.Join(outputStrs, ", "))
}

// CircuitDefinition represents the complete circuit definition
type CircuitDefinition struct {
	PrivateLen uint64      `json:"private_len"`
	PublicLen  uint64      `json:"public_len"`
	LocalLen   uint64      `json:"local_len"`
	Operations []Operation `json:"operations"`
}

// String returns a string representation of the circuit definition
func (cd *CircuitDefinition) String() string {
	return fmt.Sprintf("CircuitDefinition{private_len: %d, public_len: %d, local_len: %d, operations: %d}",
		cd.PrivateLen, cd.PublicLen, cd.LocalLen, len(cd.Operations))
}

// ParseCircuitDefinition parses a JSON string into a CircuitDefinition
func ParseCircuitDefinition(jsonData []byte) (*CircuitDefinition, error) {
	var cd CircuitDefinition
	if err := json.Unmarshal(jsonData, &cd); err != nil {
		return nil, fmt.Errorf("failed to parse circuit definition: %w", err)
	}
	return &cd, nil
}
