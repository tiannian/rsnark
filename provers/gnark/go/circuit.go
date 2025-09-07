package main

import (
	"fmt"
	"sync"

	"github.com/consensys/gnark/frontend"
)

// Global circuit definition storage
var (
	// globalCircuitDefinition stores the current circuit definition
	globalCircuitDefinition *CircuitDefinition
	// mutex protects concurrent access to the global circuit definition
	circuitMutex sync.RWMutex
)

// setCircuitDefine parses a JSON string and sets the global circuit definition
func setCircuitDefine(jsonData string) error {
	// Parse the JSON string into a CircuitDefinition
	cd, err := ParseCircuitDefinition([]byte(jsonData))
	if err != nil {
		return fmt.Errorf("failed to parse circuit definition: %w", err)
	}

	// Set the global circuit definition with thread safety
	circuitMutex.Lock()
	globalCircuitDefinition = cd
	circuitMutex.Unlock()

	return nil
}

// isCircuitDefineSet returns true if a circuit definition has been set
func isCircuitDefineSet() bool {
	circuitMutex.RLock()
	defer circuitMutex.RUnlock()
	return globalCircuitDefinition != nil
}

// clearCircuitDefine clears the global circuit definition
func clearCircuitDefine() {
	circuitMutex.Lock()
	globalCircuitDefinition = nil
	circuitMutex.Unlock()
}

// TemplateCircuit represents a gnark circuit template
type TemplateCircuit struct {
	PublicVariables  []frontend.Variable `gnark:",public"`
	PrivateVariables []frontend.Variable `gnark:",secret"`
}

// Define implements the gnark Circuit interface
func (circuit *TemplateCircuit) Define(api frontend.API) error {
	// Get the global circuit definition
	circuitMutex.RLock()
	cd := globalCircuitDefinition
	circuitMutex.RUnlock()

	if cd == nil {
		return fmt.Errorf("no circuit definition set")
	}

	// Initialize local variables array based on LocalLen
	localVariables := make([]frontend.Variable, cd.LocalLen)

	// Execute operations in order
	for i, operation := range cd.Operations {
		if err := executeOperation(api, operation, circuit.PublicVariables, circuit.PrivateVariables, &localVariables); err != nil {
			return fmt.Errorf("failed to execute operation %d (%s): %w", i, operation.Op, err)
		}
	}

	return nil
}

// NewTemplateCircuit creates a new TemplateCircuit based on the global CircuitDefinition
func NewTemplateCircuit() (*TemplateCircuit, error) {
	circuitMutex.RLock()
	cd := globalCircuitDefinition
	circuitMutex.RUnlock()

	if cd == nil {
		return nil, fmt.Errorf("no circuit definition set")
	}

	return &TemplateCircuit{
		PublicVariables:  make([]frontend.Variable, cd.PublicLen),
		PrivateVariables: make([]frontend.Variable, cd.PrivateLen),
	}, nil
}

// executeOperation executes a single operation using the gnark API
func executeOperation(api frontend.API, op Operation, publicVars, privateVars []frontend.Variable, localVars *[]frontend.Variable) error {
	// Resolve input variables
	inputs, err := resolveVariables(op.Inputs, publicVars, privateVars, *localVars)
	if err != nil {
		return fmt.Errorf("failed to resolve input variables: %w", err)
	}

	// Execute the operation based on OpCode
	var results []frontend.Variable

	switch op.Op {
	case OpAdd:
		if len(inputs) < 2 {
			return fmt.Errorf("add operation requires at least 2 inputs, got %d", len(inputs))
		}
		// api.Add supports multiple inputs: Add(a, b, c, d, ...)
		var result frontend.Variable
		if len(inputs) == 2 {
			result = api.Add(inputs[0], inputs[1])
		} else {
			result = api.Add(inputs[0], inputs[1], inputs[2:]...)
		}
		results = []frontend.Variable{result}

	case OpSub:
		if len(inputs) != 2 {
			return fmt.Errorf("sub operation requires exactly 2 inputs, got %d", len(inputs))
		}
		result := api.Sub(inputs[0], inputs[1])
		results = []frontend.Variable{result}

	case OpMul:
		if len(inputs) < 2 {
			return fmt.Errorf("mul operation requires at least 2 inputs, got %d", len(inputs))
		}
		// api.Mul supports multiple inputs: Mul(a, b, c, d, ...)
		var result frontend.Variable
		if len(inputs) == 2 {
			result = api.Mul(inputs[0], inputs[1])
		} else {
			result = api.Mul(inputs[0], inputs[1], inputs[2:]...)
		}
		results = []frontend.Variable{result}

	case OpNeg:
		if len(inputs) != 1 {
			return fmt.Errorf("neg operation requires exactly 1 input, got %d", len(inputs))
		}
		result := api.Sub(0, inputs[0]) // Negate by subtracting from 0
		results = []frontend.Variable{result}

	case OpDiv:
		if len(inputs) != 2 {
			return fmt.Errorf("div operation requires exactly 2 inputs, got %d", len(inputs))
		}
		result := api.Div(inputs[0], inputs[1])
		results = []frontend.Variable{result}

	case OpInverse:
		if len(inputs) != 1 {
			return fmt.Errorf("inverse operation requires exactly 1 input, got %d", len(inputs))
		}
		result := api.Inverse(inputs[0])
		results = []frontend.Variable{result}

	case OpToBinary:
		if len(inputs) != 1 {
			return fmt.Errorf("to_binary operation requires exactly 1 input, got %d", len(inputs))
		}
		// ToBinary can have multiple outputs based on the number of bits
		// We need to determine the number of output bits from op.Outputs length
		numBits := len(op.Outputs)
		if numBits == 0 {
			return fmt.Errorf("to_binary operation requires at least 1 output bit")
		}
		// Convert to binary with specified number of bits
		bits := api.ToBinary(inputs[0], numBits)
		results = bits

	case OpFromBinary:
		if len(inputs) == 0 {
			return fmt.Errorf("from_binary operation requires at least 1 input, got %d", len(inputs))
		}
		// FromBinary takes multiple binary inputs and produces one output
		result := api.FromBinary(inputs...)
		results = []frontend.Variable{result}

	case OpMulAcc:
		// MulAcc: multiply-accumulate operation
		// Typically: MulAcc(acc, a, b) = acc + a * b
		if len(inputs) != 3 {
			return fmt.Errorf("mul_acc operation requires exactly 3 inputs (acc, a, b), got %d", len(inputs))
		}
		product := api.Mul(inputs[1], inputs[2])
		result := api.Add(inputs[0], product)
		results = []frontend.Variable{result}

	case OpXor:
		if len(inputs) != 2 {
			return fmt.Errorf("xor operation requires exactly 2 inputs, got %d", len(inputs))
		}
		result := api.Xor(inputs[0], inputs[1])
		results = []frontend.Variable{result}

	case OpOr:
		if len(inputs) != 2 {
			return fmt.Errorf("or operation requires exactly 2 inputs, got %d", len(inputs))
		}
		result := api.Or(inputs[0], inputs[1])
		results = []frontend.Variable{result}

	case OpAnd:
		if len(inputs) != 2 {
			return fmt.Errorf("and operation requires exactly 2 inputs, got %d", len(inputs))
		}
		result := api.And(inputs[0], inputs[1])
		results = []frontend.Variable{result}

	case OpSelect:
		if len(inputs) != 3 {
			return fmt.Errorf("select operation requires exactly 3 inputs (condition, ifTrue, ifFalse), got %d", len(inputs))
		}
		result := api.Select(inputs[0], inputs[1], inputs[2])
		results = []frontend.Variable{result}

	case OpIsZero:
		if len(inputs) != 1 {
			return fmt.Errorf("is_zero operation requires exactly 1 input, got %d", len(inputs))
		}
		result := api.IsZero(inputs[0])
		results = []frontend.Variable{result}

	case OpCmp:
		if len(inputs) != 2 {
			return fmt.Errorf("cmp operation requires exactly 2 inputs, got %d", len(inputs))
		}
		result := api.Cmp(inputs[0], inputs[1])
		results = []frontend.Variable{result}

	case OpAssertIsEqual:
		if len(inputs) != 2 {
			return fmt.Errorf("assert_is_equal operation requires exactly 2 inputs, got %d", len(inputs))
		}
		api.AssertIsEqual(inputs[0], inputs[1])
		results = nil // No output

	case OpAssertIsDifferent:
		if len(inputs) != 2 {
			return fmt.Errorf("assert_is_different operation requires exactly 2 inputs, got %d", len(inputs))
		}
		// gnark doesn't have AssertIsDifferent directly, we can implement it as:
		// AssertIsEqual(Mul(Sub(a, b), Inverse(Sub(a, b))), 1)
		diff := api.Sub(inputs[0], inputs[1])
		api.AssertIsEqual(api.Mul(diff, api.Inverse(diff)), 1)
		results = nil // No output

	case OpAssertIsBoolean:
		if len(inputs) != 1 {
			return fmt.Errorf("assert_is_boolean operation requires exactly 1 input, got %d", len(inputs))
		}
		api.AssertIsBoolean(inputs[0])
		results = nil // No output

	default:
		return fmt.Errorf("unsupported operation: %s", op.Op)
	}

	// Assign results to output variables
	if len(results) > 0 {
		if len(op.Outputs) != len(results) {
			return fmt.Errorf("operation %s produced %d results but has %d outputs", op.Op, len(results), len(op.Outputs))
		}

		for i, result := range results {
			output := op.Outputs[i]
			if err := assignVariable(result, output, publicVars, privateVars, localVars); err != nil {
				return fmt.Errorf("failed to assign output variable %d: %w", i, err)
			}
		}
	} else {
		if len(op.Outputs) != 0 {
			return fmt.Errorf("operation %s should have no outputs, got %d", op.Op, len(op.Outputs))
		}
	}

	return nil
}

// resolveVariables converts VariableType to frontend.Variable
func resolveVariables(varTypes []VariableType, publicVars, privateVars, localVars []frontend.Variable) ([]frontend.Variable, error) {
	variables := make([]frontend.Variable, len(varTypes))

	for i, vt := range varTypes {
		switch vt.Type {
		case string(VarPublic):
			if vt.ValueInt == nil {
				return nil, fmt.Errorf("public variable missing index")
			}
			index := *vt.ValueInt
			if index >= uint64(len(publicVars)) {
				return nil, fmt.Errorf("public variable index %d out of bounds (max %d)", index, len(publicVars)-1)
			}
			variables[i] = publicVars[index]

		case string(VarPrivate):
			if vt.ValueInt == nil {
				return nil, fmt.Errorf("private variable missing index")
			}
			index := *vt.ValueInt
			if index >= uint64(len(privateVars)) {
				return nil, fmt.Errorf("private variable index %d out of bounds (max %d)", index, len(privateVars)-1)
			}
			variables[i] = privateVars[index]

		case string(VarLocal):
			if vt.ValueInt == nil {
				return nil, fmt.Errorf("local variable missing index")
			}
			index := *vt.ValueInt
			if index >= uint64(len(localVars)) {
				return nil, fmt.Errorf("local variable index %d out of bounds (max %d)", index, len(localVars)-1)
			}
			variables[i] = localVars[index]

		case string(VarConstant):
			if vt.ValueBigInt == nil {
				return nil, fmt.Errorf("constant variable missing value")
			}
			variables[i] = vt.ValueBigInt

		default:
			return nil, fmt.Errorf("unknown variable type: %s", vt.Type)
		}
	}

	return variables, nil
}

// assignVariable assigns a value to a variable based on its type
func assignVariable(value frontend.Variable, varType VariableType, publicVars, privateVars []frontend.Variable, localVars *[]frontend.Variable) error {
	switch varType.Type {
	case string(VarPublic):
		if varType.ValueInt == nil {
			return fmt.Errorf("public variable missing index")
		}
		index := *varType.ValueInt
		if index >= uint64(len(publicVars)) {
			return fmt.Errorf("public variable index %d out of bounds (max %d)", index, len(publicVars)-1)
		}
		publicVars[index] = value

	case string(VarPrivate):
		if varType.ValueInt == nil {
			return fmt.Errorf("private variable missing index")
		}
		index := *varType.ValueInt
		if index >= uint64(len(privateVars)) {
			return fmt.Errorf("private variable index %d out of bounds (max %d)", index, len(privateVars)-1)
		}
		privateVars[index] = value

	case string(VarLocal):
		if varType.ValueInt == nil {
			return fmt.Errorf("local variable missing index")
		}
		index := *varType.ValueInt
		if index >= uint64(len(*localVars)) {
			return fmt.Errorf("local variable index %d out of bounds (max %d)", index, len(*localVars)-1)
		}
		(*localVars)[index] = value

	default:
		return fmt.Errorf("cannot assign to variable type: %s", varType.Type)
	}

	return nil
}
