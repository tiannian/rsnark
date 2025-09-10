package types

import (
	"bytes"
	"fmt"

	"github.com/consensys/gnark/backend/groth16"
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
