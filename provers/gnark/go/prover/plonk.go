package prover

import (
	"fmt"

	"github.com/consensys/gnark/backend/plonk"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/scs"
	"github.com/consensys/gnark/test/unsafekzg"

	"github.com/tiannian/rsnark/provers-gnark/circuit"
	"github.com/tiannian/rsnark/provers-gnark/prover/types"
)

// PlonkProver represents the PLONK prover interface
type PlonkProver struct {
	curve types.CurveType
}

// NewPlonkProver creates a new PLONK prover instance
func NewPlonkProver(curve types.CurveType) *PlonkProver {
	return &PlonkProver{
		curve: curve,
	}
}

func (p *PlonkProver) CurveId() types.CurveType {
	return p.curve
}

// Compile compiles a circuit from CircuitDefinition
func (p *PlonkProver) Compile(cd *circuit.CircuitDefinition) (*types.PlonkCompiledCircuit, error) {
	// Create TemplateCircuit from CircuitDefinition
	templateCircuit, err := circuit.NewTemplateCircuit(cd)
	if err != nil {
		return nil, fmt.Errorf("failed to create template circuit: %w", err)
	}

	// Compile to SCS for PLONK (Sparse Constraint System)
	scs, err := frontend.Compile(p.curve.ToECC().ScalarField(), scs.NewBuilder, templateCircuit)
	if err != nil {
		return nil, fmt.Errorf("failed to compile circuit to SCS: %w", err)
	}

	compiled := &types.PlonkCompiledCircuit{
		CS: scs,
	}

	return compiled, nil
}

// Setup performs the trusted setup for the compiled circuit
func (p *PlonkProver) Setup(compiled *types.PlonkCompiledCircuit) (*types.PlonkProvingKey, *types.PlonkVerifyingKey, error) {

	srs, srsLagrange, err := unsafekzg.NewSRS(compiled.CS)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to create KZG SRS: %w", err)
	}

	pk, vk, err := plonk.Setup(compiled.CS, srs, srsLagrange)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to setup PLONK: %w", err)
	}

	provingKey := types.NewPlonkProvingKey()
	provingKey.Key = pk

	verifyingKey := types.NewPlonkVerifyingKey()
	verifyingKey.Key = vk

	return provingKey, verifyingKey, nil
}

// Prove generates a proof for the given TemplateWitness and returns serialized proof bytes
func (p *PlonkProver) Prove(compiled *types.PlonkCompiledCircuit, pk *types.PlonkProvingKey, witness *types.TemplateWitness) (*types.PlonkProof, error) {
	// Create gnark witness from TemplateWitness
	// For now, we create a circuit copy and assign values
	circuitCopy := &circuit.TemplateCircuit{
		PublicVariables:  make([]frontend.Variable, len(witness.PublicVariables)),
		PrivateVariables: make([]frontend.Variable, len(witness.PrivateVariables)),
	}

	// Assign public variables
	for i, val := range witness.PublicVariables {
		circuitCopy.PublicVariables[i] = val
	}

	// Assign private variables
	for i, val := range witness.PrivateVariables {
		circuitCopy.PrivateVariables[i] = val
	}

	// Create gnark witness
	gnarkWitness, err := frontend.NewWitness(circuitCopy, p.curve.ToECC().ScalarField())
	if err != nil {
		return nil, fmt.Errorf("failed to create gnark witness: %w", err)
	}

	plonkProof, err := plonk.Prove(compiled.CS, pk.Key, gnarkWitness)
	if err != nil {
		return nil, fmt.Errorf("failed to generate PLONK proof: %w", err)
	}

	return &types.PlonkProof{
		Proof: plonkProof,
	}, nil
}

// Verify verifies a proof against the public witness
func (p *PlonkProver) Verify(proof *types.PlonkProof, vk *types.PlonkVerifyingKey, publicWitness *types.TemplatePublicWitness) error {
	// Create a template circuit with only public variables for verification
	templateCircuit := &circuit.TemplateCircuit{
		PublicVariables: make([]frontend.Variable, len(publicWitness.PublicVariables)),
		// I think this initialization is unnecessary, but it's here to avoid warnings. I don't know why it's needed.
		PrivateVariables: make([]frontend.Variable, 1),
	}

	// Assign public variables
	for i, val := range publicWitness.PublicVariables {
		templateCircuit.PublicVariables[i] = val
	}

	// Create gnark public witness
	gnarkWitness, err := frontend.NewWitness(templateCircuit, p.curve.ToECC().ScalarField(), frontend.PublicOnly())
	if err != nil {
		return fmt.Errorf("failed to create gnark witness: %w", err)
	}
	publicGnarkWitness, err := gnarkWitness.Public()
	if err != nil {
		return fmt.Errorf("failed to extract public witness: %w", err)
	}

	err = plonk.Verify(proof.Proof, vk.Key, publicGnarkWitness)
	if err != nil {
		return fmt.Errorf("PLONK verification failed: %w", err)
	}

	return nil
}
