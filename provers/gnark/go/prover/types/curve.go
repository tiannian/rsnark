package types

import "github.com/consensys/gnark-crypto/ecc"

// CurveType represents the supported elliptic curves
type CurveType int

const (
	CurveBN254 CurveType = iota
	CurveBLS12381
	CurveBLS12377
	CurveBW6761
)

// String returns the string representation of the curve type
func (c CurveType) String() string {
	switch c {
	case CurveBN254:
		return "bn254"
	case CurveBLS12381:
		return "bls12-381"
	case CurveBLS12377:
		return "bls12-377"
	case CurveBW6761:
		return "bw6-761"
	default:
		return "unknown"
	}
}

// ToECC converts CurveType to gnark's ecc.ID
func (c CurveType) ToECC() ecc.ID {
	switch c {
	case CurveBN254:
		return ecc.BN254
	case CurveBLS12381:
		return ecc.BLS12_381
	case CurveBLS12377:
		return ecc.BLS12_377
	case CurveBW6761:
		return ecc.BW6_761
	default:
		return ecc.BN254 // default to BN254
	}
}
