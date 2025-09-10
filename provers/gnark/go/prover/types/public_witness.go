package types

import (
	"encoding/json"
	"math/big"
)

// TemplatePublicWitness represents only the public part of the witness
type TemplatePublicWitness struct {
	PublicVariables []*big.Int `json:"public_variables"`
}

// NewTemplatePublicWitness creates a new TemplatePublicWitness
func NewTemplatePublicWitness(publicVars []*big.Int) *TemplatePublicWitness {
	return &TemplatePublicWitness{
		PublicVariables: publicVars,
	}
}

// NewTemplatePublicWitnessFromTemplate creates a new TemplatePublicWitness from a TemplateWitness
func NewTemplatePublicWitnessFromTemplate(w *TemplateWitness) *TemplatePublicWitness {
	return &TemplatePublicWitness{
		PublicVariables: w.PublicVariables,
	}
}

// ToJSON serializes the public witness to JSON
func (pw *TemplatePublicWitness) ToJSON() ([]byte, error) {
	return json.Marshal(pw)
}

// FromJSON deserializes the public witness from JSON
func (pw *TemplatePublicWitness) FromJSON(data []byte) error {
	return json.Unmarshal(data, pw)
}
