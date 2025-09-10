package types

import (
	"encoding/json"
	"fmt"
	"math/big"
)

// TemplateWitness represents the witness data for the circuit
// Contains both public and private variables as big.Int arrays
type TemplateWitness struct {
	PublicVariables  []*big.Int `json:"public_variables"`
	PrivateVariables []*big.Int `json:"private_variables"`
}

// NewTemplateWitness creates a new TemplateWitness
func NewTemplateWitness(publicVars, privateVars []*big.Int) *TemplateWitness {
	return &TemplateWitness{
		PublicVariables:  publicVars,
		PrivateVariables: privateVars,
	}
}

// ToJSON serializes the witness to JSON
func (w *TemplateWitness) ToJSON() ([]byte, error) {
	return json.Marshal(w)
}

// FromJSON deserializes the witness from JSON
func (w *TemplateWitness) FromJSON(data []byte) error {
	return json.Unmarshal(data, w)
}

// ToGnarkWitness converts TemplateWitness to gnark witness format
func (w *TemplateWitness) ToGnarkWitness(templateCircuit interface{}, curve CurveType) (interface{}, error) {
	// This is a placeholder - in practice, you would need to properly assign values
	// to the circuit's public and private variables based on the big.Int arrays
	// The actual implementation would depend on how gnark witness assignment works
	return nil, fmt.Errorf("ToGnarkWitness not implemented yet")
}
