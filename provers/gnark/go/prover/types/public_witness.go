package types

import (
	"encoding/json"
	"fmt"
	"math/big"
	"strings"
)

// TemplatePublicWitness represents only the public part of the witness
type TemplatePublicWitness struct {
	PublicVariables []*big.Int `json:"public"`
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

// parseHexStringToBigIntPW converts hex string (0x format) to *big.Int for public witness
func parseHexStringToBigIntPW(hexStr string) (*big.Int, error) {
	if strings.HasPrefix(hexStr, "0x") || strings.HasPrefix(hexStr, "0X") {
		hexStr = hexStr[2:] // Remove 0x prefix
	}
	bigInt := new(big.Int)
	bigInt, ok := bigInt.SetString(hexStr, 16)
	if !ok {
		return nil, fmt.Errorf("invalid hex string: %s", hexStr)
	}
	return bigInt, nil
}

// templatePublicWitnessJSON is a helper struct for JSON unmarshaling
type templatePublicWitnessJSON struct {
	PublicVariables []interface{} `json:"public"`
}

// FromJSON deserializes the public witness from JSON, supporting both *big.Int and hex strings
func (pw *TemplatePublicWitness) FromJSON(data []byte) error {
	var temp templatePublicWitnessJSON
	if err := json.Unmarshal(data, &temp); err != nil {
		return err
	}

	// Parse public variables
	pw.PublicVariables = make([]*big.Int, len(temp.PublicVariables))
	for i, v := range temp.PublicVariables {
		switch val := v.(type) {
		case string:
			bigInt, err := parseHexStringToBigIntPW(val)
			if err != nil {
				return fmt.Errorf("error parsing public variable %d: %v", i, err)
			}
			pw.PublicVariables[i] = bigInt
		case float64:
			pw.PublicVariables[i] = big.NewInt(int64(val))
		default:
			return fmt.Errorf("unsupported type for public variable %d: %T", i, v)
		}
	}

	return nil
}
