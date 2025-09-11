package types

import (
	"encoding/json"
	"fmt"
	"math/big"
	"strings"
)

// TemplateWitness represents the witness data for the circuit
// Contains both public and private variables as big.Int arrays
type TemplateWitness struct {
	PublicVariables  []*big.Int `json:"public"`
	PrivateVariables []*big.Int `json:"private"`
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

// parseHexStringToBigInt converts hex string (0x format) to *big.Int
func parseHexStringToBigInt(hexStr string) (*big.Int, error) {
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

// templateWitnessJSON is a helper struct for JSON unmarshaling
type templateWitnessJSON struct {
	PublicVariables  []interface{} `json:"public"`
	PrivateVariables []interface{} `json:"private"`
}

// FromJSON deserializes the witness from JSON, supporting both *big.Int and hex strings
func (w *TemplateWitness) FromJSON(data []byte) error {
	var temp templateWitnessJSON
	if err := json.Unmarshal(data, &temp); err != nil {
		return err
	}

	// Parse public variables
	w.PublicVariables = make([]*big.Int, len(temp.PublicVariables))
	for i, v := range temp.PublicVariables {
		switch val := v.(type) {
		case string:
			bigInt, err := parseHexStringToBigInt(val)
			if err != nil {
				return fmt.Errorf("error parsing public variable %d: %v", i, err)
			}
			w.PublicVariables[i] = bigInt
		case float64:
			w.PublicVariables[i] = big.NewInt(int64(val))
		default:
			return fmt.Errorf("unsupported type for public variable %d: %T", i, v)
		}
	}

	// Parse private variables
	w.PrivateVariables = make([]*big.Int, len(temp.PrivateVariables))
	for i, v := range temp.PrivateVariables {
		switch val := v.(type) {
		case string:
			bigInt, err := parseHexStringToBigInt(val)
			if err != nil {
				return fmt.Errorf("error parsing private variable %d: %v", i, err)
			}
			w.PrivateVariables[i] = bigInt
		case float64:
			w.PrivateVariables[i] = big.NewInt(int64(val))
		default:
			return fmt.Errorf("unsupported type for private variable %d: %T", i, v)
		}
	}

	return nil
}

// ToGnarkWitness converts TemplateWitness to gnark witness format
func (w *TemplateWitness) ToGnarkWitness(templateCircuit interface{}, curve CurveType) (interface{}, error) {
	// This is a placeholder - in practice, you would need to properly assign values
	// to the circuit's public and private variables based on the big.Int arrays
	// The actual implementation would depend on how gnark witness assignment works
	return nil, fmt.Errorf("ToGnarkWitness not implemented yet")
}
