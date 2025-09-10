package types

import (
	"bytes"
	"fmt"

	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/constraint"
)

// CompiledCircuit represents a compiled circuit (can be used for both Groth16 and PLONK)
type CompiledCircuit struct {
	CS constraint.ConstraintSystem // Generic constraint system (R1CS or SCS)
}

// NewCompiledCircuit creates a new CompiledCircuit
func NewCompiledCircuit() *CompiledCircuit {
	return &CompiledCircuit{}
}

// Serialize serializes the compiled circuit to bytes
func (cc *CompiledCircuit) Serialize() ([]byte, error) {
	var buf bytes.Buffer
	_, err := cc.CS.WriteTo(&buf)
	if err != nil {
		return nil, fmt.Errorf("failed to serialize compiled circuit: %w", err)
	}
	return buf.Bytes(), nil
}

// Deserialize deserializes the compiled circuit from bytes
func (cc *CompiledCircuit) Deserialize(data []byte, curve CurveType) error {
	// Initialize a new constraint system based on the curve using groth16.NewCS
	cs := groth16.NewCS(curve.ToECC())

	buf := bytes.NewReader(data)
	_, err := cs.ReadFrom(buf)
	if err != nil {
		return fmt.Errorf("failed to deserialize compiled circuit: %w", err)
	}

	cc.CS = cs
	return nil
}
