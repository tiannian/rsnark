package types

import "github.com/consensys/gnark-crypto/ecc"

// CurveType represents the supported elliptic curves
type CurveType int

const (
	CurveBN254     CurveType = 1
	CurveBLS12_381 CurveType = 2
	CurveBLS24_317 CurveType = 3
	CurveBLS12_377 CurveType = 4
	CurveBW6_761   CurveType = 5
	CurveBLS24_315 CurveType = 6
	CurveBW6_633   CurveType = 7
)

// ToECC converts CurveType to gnark's ecc.ID
func (c CurveType) ToECC() ecc.ID {
	switch c {
	case CurveBN254:
		return ecc.BN254
	case CurveBLS12_381:
		return ecc.BLS12_381
	case CurveBLS12_377:
		return ecc.BLS12_377
	case CurveBW6_761:
		return ecc.BW6_761
	case CurveBLS24_315:
		return ecc.BLS24_315
	case CurveBLS24_317:
		return ecc.BLS24_317
	case CurveBW6_633:
		return ecc.BW6_633
	default:
		return ecc.BN254 // default to BN254
	}
}
