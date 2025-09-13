package types

import (
	"bytes"
	"fmt"

	"github.com/consensys/gnark/backend/plonk"
	"github.com/consensys/gnark/constraint"
)

// PlonkProvingKey wraps gnark PLONK proving key with basic serialization
type PlonkProvingKey struct {
	Key plonk.ProvingKey
}

// NewPlonkProvingKey creates a new PlonkProvingKey
func NewPlonkProvingKey() *PlonkProvingKey {
	return &PlonkProvingKey{}
}

// Serialize serializes the proving key to bytes
func (pk *PlonkProvingKey) Serialize() ([]byte, error) {
	var buf bytes.Buffer
	_, err := pk.Key.WriteTo(&buf)
	if err != nil {
		return nil, fmt.Errorf("failed to serialize PLONK proving key: %w", err)
	}
	return buf.Bytes(), nil
}

// Deserialize deserializes the proving key from bytes
func (pk *PlonkProvingKey) Deserialize(data []byte, curve CurveType) error {
	// Initialize the key based on the curve
	pk.Key = plonk.NewProvingKey(curve.ToECC())

	buf := bytes.NewReader(data)
	_, err := pk.Key.ReadFrom(buf)
	if err != nil {
		return fmt.Errorf("failed to deserialize PLONK proving key: %w", err)
	}
	return nil
}

// PlonkVerifyingKey wraps gnark PLONK verifying key with basic serialization
type PlonkVerifyingKey struct {
	Key plonk.VerifyingKey
}

// NewPlonkVerifyingKey creates a new PlonkVerifyingKey
func NewPlonkVerifyingKey() *PlonkVerifyingKey {
	return &PlonkVerifyingKey{}
}

// Serialize serializes the verifying key to bytes
func (vk *PlonkVerifyingKey) Serialize() ([]byte, error) {
	var buf bytes.Buffer
	_, err := vk.Key.WriteTo(&buf)
	if err != nil {
		return nil, fmt.Errorf("failed to serialize PLONK verifying key: %w", err)
	}
	return buf.Bytes(), nil
}

// Deserialize deserializes the verifying key from bytes
func (vk *PlonkVerifyingKey) Deserialize(data []byte, curve CurveType) error {
	// Initialize the key based on the curve
	vk.Key = plonk.NewVerifyingKey(curve.ToECC())

	buf := bytes.NewReader(data)
	_, err := vk.Key.ReadFrom(buf)
	if err != nil {
		return fmt.Errorf("failed to deserialize PLONK verifying key: %w", err)
	}
	return nil
}

// ExportSolidity exports the verifying key to Solidity format (if supported by gnark PLONK)
func (vk *PlonkVerifyingKey) ExportSolidity() ([]byte, error) {
	var buf bytes.Buffer

	err := vk.Key.ExportSolidity(&buf)
	if err != nil {
		return nil, fmt.Errorf("failed to export PLONK verifying key to solidity: %w", err)
	}

	return buf.Bytes(), nil
}

// CompiledCircuit represents a compiled circuit
type PlonkCompiledCircuit struct {
	CS constraint.ConstraintSystem // Generic constraint system (SCS)
}

// NewCompiledCircuit creates a new CompiledCircuit
func NewPlonkCompiledCircuit() *PlonkCompiledCircuit {
	return &PlonkCompiledCircuit{}
}

// Serialize serializes the compiled circuit to bytes
func (cc *PlonkCompiledCircuit) Serialize() ([]byte, error) {
	var buf bytes.Buffer
	_, err := cc.CS.WriteTo(&buf)
	if err != nil {
		return nil, fmt.Errorf("failed to serialize compiled circuit: %w", err)
	}
	return buf.Bytes(), nil
}

// Deserialize deserializes the compiled circuit from bytes
func (cc *PlonkCompiledCircuit) Deserialize(data []byte, curve CurveType) error {
	// Initialize a new constraint system based on the curve using plonk.NewCS
	cs := plonk.NewCS(curve.ToECC())

	buf := bytes.NewReader(data)
	_, err := cs.ReadFrom(buf)
	if err != nil {
		return fmt.Errorf("failed to deserialize compiled circuit: %w", err)
	}

	cc.CS = cs
	return nil
}

// PlonkProof represents a PLONK proof
type PlonkProof struct {
	Proof plonk.Proof
}

// NewPlonkProof creates a new PlonkProof
func NewPlonkProof() *PlonkProof {
	return &PlonkProof{}
}

// Serialize serializes the proof to bytes
func (p *PlonkProof) Serialize() ([]byte, error) {
	var buf bytes.Buffer
	_, err := p.Proof.WriteTo(&buf)
	if err != nil {
		return nil, fmt.Errorf("failed to serialize proof: %w", err)
	}
	return buf.Bytes(), nil
}

// Deserialize deserializes the proof from bytes
func (p *PlonkProof) Deserialize(data []byte, curve CurveType) error {
	p.Proof = plonk.NewProof(curve.ToECC())

	buf := bytes.NewReader(data)
	_, err := p.Proof.ReadFrom(buf)
	if err != nil {
		return fmt.Errorf("failed to deserialize proof: %w", err)
	}
	return nil
}
