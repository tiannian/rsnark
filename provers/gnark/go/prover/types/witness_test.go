package types

import (
	"math/big"
	"testing"
)

func TestTemplateWitnessFromJSONWithHexStrings(t *testing.T) {
	// Test data with hex strings
	testJSON := `{
		"public": ["0x1a", "0x2b", "0xFF"],
		"private": ["0x123", "0xabc", "0x456def"]
	}`

	var witness TemplateWitness
	err := witness.FromJSON([]byte(testJSON))
	if err != nil {
		t.Fatalf("Error parsing TemplateWitness with hex strings: %v", err)
	}

	// Check public variables
	expectedPublic := []*big.Int{
		big.NewInt(26),  // 0x1a
		big.NewInt(43),  // 0x2b
		big.NewInt(255), // 0xFF
	}

	if len(witness.PublicVariables) != len(expectedPublic) {
		t.Fatalf("Expected %d public variables, got %d", len(expectedPublic), len(witness.PublicVariables))
	}

	for i, expected := range expectedPublic {
		if witness.PublicVariables[i].Cmp(expected) != 0 {
			t.Errorf("Public variable %d: expected %s, got %s", i, expected.String(), witness.PublicVariables[i].String())
		}
	}

	// Check private variables
	expectedPrivate := []*big.Int{
		big.NewInt(291),     // 0x123
		big.NewInt(2748),    // 0xabc
		big.NewInt(4550127), // 0x456def
	}

	if len(witness.PrivateVariables) != len(expectedPrivate) {
		t.Fatalf("Expected %d private variables, got %d", len(expectedPrivate), len(witness.PrivateVariables))
	}

	for i, expected := range expectedPrivate {
		if witness.PrivateVariables[i].Cmp(expected) != 0 {
			t.Errorf("Private variable %d: expected %s, got %s", i, expected.String(), witness.PrivateVariables[i].String())
		}
	}
}

func TestTemplatePublicWitnessFromJSONWithHexStrings(t *testing.T) {
	// Test data with hex strings
	testJSON := `{
		"public": ["0x1a", "0x2b", "0xFF", "0x123456789abcdef"]
	}`

	var publicWitness TemplatePublicWitness
	err := publicWitness.FromJSON([]byte(testJSON))
	if err != nil {
		t.Fatalf("Error parsing TemplatePublicWitness with hex strings: %v", err)
	}

	// Check public variables
	expectedPublic := []*big.Int{
		big.NewInt(26),  // 0x1a
		big.NewInt(43),  // 0x2b
		big.NewInt(255), // 0xFF
	}

	// Add the large hex number
	largeHex := new(big.Int)
	largeHex.SetString("123456789abcdef", 16)
	expectedPublic = append(expectedPublic, largeHex)

	if len(publicWitness.PublicVariables) != len(expectedPublic) {
		t.Fatalf("Expected %d public variables, got %d", len(expectedPublic), len(publicWitness.PublicVariables))
	}

	for i, expected := range expectedPublic {
		if publicWitness.PublicVariables[i].Cmp(expected) != 0 {
			t.Errorf("Public variable %d: expected %s, got %s", i, expected.String(), publicWitness.PublicVariables[i].String())
		}
	}
}

func TestMixedFormatFromJSON(t *testing.T) {
	// Test with mixed hex strings and numbers
	testJSON := `{
		"public": ["0x1a", 100, "0xFF"],
		"private": ["0x123", 200]
	}`

	var witness TemplateWitness
	err := witness.FromJSON([]byte(testJSON))
	if err != nil {
		t.Fatalf("Error parsing mixed format: %v", err)
	}

	// Check public variables
	expectedPublic := []*big.Int{
		big.NewInt(26),  // 0x1a
		big.NewInt(100), // 100
		big.NewInt(255), // 0xFF
	}

	if len(witness.PublicVariables) != len(expectedPublic) {
		t.Fatalf("Expected %d public variables, got %d", len(expectedPublic), len(witness.PublicVariables))
	}

	for i, expected := range expectedPublic {
		if witness.PublicVariables[i].Cmp(expected) != 0 {
			t.Errorf("Public variable %d: expected %s, got %s", i, expected.String(), witness.PublicVariables[i].String())
		}
	}

	// Check private variables
	expectedPrivate := []*big.Int{
		big.NewInt(291), // 0x123
		big.NewInt(200), // 200
	}

	if len(witness.PrivateVariables) != len(expectedPrivate) {
		t.Fatalf("Expected %d private variables, got %d", len(expectedPrivate), len(witness.PrivateVariables))
	}

	for i, expected := range expectedPrivate {
		if witness.PrivateVariables[i].Cmp(expected) != 0 {
			t.Errorf("Private variable %d: expected %s, got %s", i, expected.String(), witness.PrivateVariables[i].String())
		}
	}
}

func TestInvalidHexString(t *testing.T) {
	// Test with invalid hex string
	testJSON := `{
		"public": ["0xZZ", "0x1a"],
		"private": []
	}`

	var witness TemplateWitness
	err := witness.FromJSON([]byte(testJSON))
	if err == nil {
		t.Fatal("Expected error for invalid hex string, but got nil")
	}

	if !contains(err.Error(), "invalid hex string") {
		t.Errorf("Expected error message to contain 'invalid hex string', got: %s", err.Error())
	}
}

// Helper function to check if string contains substring
func contains(s, substr string) bool {
	return len(s) >= len(substr) && (s == substr || len(substr) == 0 || (len(s) > len(substr) && s[:len(substr)] == substr) || contains(s[1:], substr))
}
