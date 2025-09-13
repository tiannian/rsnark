package types

import (
	"bytes"
	"fmt"

	"github.com/consensys/gnark/backend/groth16"
	bn254groth16 "github.com/consensys/gnark/backend/groth16/bn254"
	"github.com/consensys/gnark/constraint"
)

// Groth16ProvingKey wraps gnark Groth16 proving key with basic serialization
type Groth16ProvingKey struct {
	Key groth16.ProvingKey
}

// NewGroth16ProvingKey creates a new Groth16ProvingKey
func NewGroth16ProvingKey() *Groth16ProvingKey {
	return &Groth16ProvingKey{}
}

// Serialize serializes the proving key to bytes
func (pk *Groth16ProvingKey) Serialize() ([]byte, error) {
	var buf bytes.Buffer
	_, err := pk.Key.WriteTo(&buf)
	if err != nil {
		return nil, fmt.Errorf("failed to serialize proving key: %w", err)
	}
	return buf.Bytes(), nil
}

// Deserialize deserializes the proving key from bytes
func (pk *Groth16ProvingKey) Deserialize(data []byte, curve CurveType) error {
	// Initialize the key based on the curve
	pk.Key = groth16.NewProvingKey(curve.ToECC())

	buf := bytes.NewReader(data)
	_, err := pk.Key.ReadFrom(buf)
	if err != nil {
		return fmt.Errorf("failed to deserialize proving key: %w", err)
	}
	return nil
}

// Groth16VerifyingKey wraps gnark Groth16 verifying key with basic serialization
type Groth16VerifyingKey struct {
	Key groth16.VerifyingKey
}

// NewGroth16VerifyingKey creates a new Groth16VerifyingKey
func NewGroth16VerifyingKey() *Groth16VerifyingKey {
	return &Groth16VerifyingKey{}
}

// Serialize serializes the verifying key to bytes
func (vk *Groth16VerifyingKey) Serialize() ([]byte, error) {
	var buf bytes.Buffer
	_, err := vk.Key.WriteTo(&buf)
	if err != nil {
		return nil, fmt.Errorf("failed to serialize verifying key: %w", err)
	}
	return buf.Bytes(), nil
}

// Deserialize deserializes the verifying key from bytes
func (vk *Groth16VerifyingKey) Deserialize(data []byte, curve CurveType) error {
	// Initialize the key based on the curve
	vk.Key = groth16.NewVerifyingKey(curve.ToECC())

	buf := bytes.NewReader(data)
	_, err := vk.Key.ReadFrom(buf)
	if err != nil {
		return fmt.Errorf("failed to deserialize verifying key: %w", err)
	}
	return nil
}

func (vk *Groth16VerifyingKey) ExportSolidity() ([]byte, error) {
	var buf bytes.Buffer

	err := vk.Key.ExportSolidity(&buf)
	if err != nil {
		return nil, fmt.Errorf("failed to export verifying key to solidity: %w", err)
	}

	return buf.Bytes(), nil
}

// CompiledCircuit represents a compiled circuit
type Groth16CompiledCircuit struct {
	CS constraint.ConstraintSystem // Generic constraint system (R1CS)
}

// NewCompiledCircuit creates a new CompiledCircuit
func NewGroth16CompiledCircuit() *Groth16CompiledCircuit {
	return &Groth16CompiledCircuit{}
}

// Serialize serializes the compiled circuit to bytes
func (cc *Groth16CompiledCircuit) Serialize() ([]byte, error) {
	var buf bytes.Buffer
	_, err := cc.CS.WriteTo(&buf)
	if err != nil {
		return nil, fmt.Errorf("failed to serialize compiled circuit: %w", err)
	}
	return buf.Bytes(), nil
}

// Deserialize deserializes the compiled circuit from bytes
func (cc *Groth16CompiledCircuit) Deserialize(data []byte, curve CurveType) error {
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

// Groth16Proof represents a Groth16 proof
type Groth16Proof struct {
	Proof groth16.Proof
}

// NewGroth16Proof creates a new Groth16Proof
func NewGroth16Proof() *Groth16Proof {
	return &Groth16Proof{}
}

// Serialize serializes the proof to bytes
func (p *Groth16Proof) Serialize() ([]byte, error) {
	var buf bytes.Buffer
	_, err := p.Proof.WriteTo(&buf)
	if err != nil {
		return nil, fmt.Errorf("failed to serialize proof: %w", err)
	}
	return buf.Bytes(), nil
}

// Deserialize deserializes the proof from bytes
func (p *Groth16Proof) Deserialize(data []byte, curve CurveType) error {
	p.Proof = groth16.NewProof(curve.ToECC())

	buf := bytes.NewReader(data)
	_, err := p.Proof.ReadFrom(buf)
	if err != nil {
		return fmt.Errorf("failed to deserialize proof: %w", err)
	}
	return nil
}

func (p *Groth16Proof) ExportSolidity() ([]byte, error) {
	bn254groth16proof, ok := p.Proof.(*bn254groth16.Proof)
	if !ok {
		return nil, fmt.Errorf("failed to cast proof to bn254groth16.Proof")
	}

	bytes := bn254groth16proof.MarshalSolidity()

	return bytes, nil
}
