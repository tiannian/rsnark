package prover

import (
	"bytes"
	"fmt"

	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"

	"github.com/tiannian/rsnark/provers-gnark/circuit"
	"github.com/tiannian/rsnark/provers-gnark/prover/types"
)

// Groth16Prover represents the Groth16 prover interface
type Groth16Prover struct {
	curve types.CurveType
}

// NewGroth16Prover creates a new Groth16 prover instance
func NewGroth16Prover(curve types.CurveType) *Groth16Prover {
	return &Groth16Prover{
		curve: curve,
	}
}

func (p *Groth16Prover) CurveId() types.CurveType {
	return p.curve
}

// CompileFromDefinition compiles a circuit from CircuitDefinition
func (p *Groth16Prover) Compile(cd *circuit.CircuitDefinition) (*types.CompiledCircuit, error) {
	// Create TemplateCircuit from CircuitDefinition
	templateCircuit, err := circuit.NewTemplateCircuit(cd)
	if err != nil {
		return nil, fmt.Errorf("failed to create template circuit: %w", err)
	}

	// Compile to R1CS for Groth16
	r1cs, err := frontend.Compile(p.curve.ToECC().ScalarField(), r1cs.NewBuilder, templateCircuit)
	if err != nil {
		return nil, fmt.Errorf("failed to compile circuit to R1CS: %w", err)
	}

	compiled := &types.CompiledCircuit{
		CS: r1cs,
	}

	return compiled, nil
}

// Setup performs the trusted setup for the compiled circuit
func (p *Groth16Prover) Setup(compiled *types.CompiledCircuit) (*types.Groth16ProvingKey, *types.Groth16VerifyingKey, error) {
	pk, vk, err := groth16.Setup(compiled.CS)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to setup Groth16: %w", err)
	}

	provingKey := types.NewGroth16ProvingKey()
	provingKey.Key = pk

	verifyingKey := types.NewGroth16VerifyingKey()
	verifyingKey.Key = vk

	return provingKey, verifyingKey, nil
}

// ProveWithTemplate generates a proof for the given TemplateWitness and returns serialized proof bytes
func (p *Groth16Prover) Prove(compiled *types.CompiledCircuit, pk *types.Groth16ProvingKey, witness *types.TemplateWitness) ([]byte, error) {
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

	groth16Proof, err := groth16.Prove(compiled.CS, pk.Key, gnarkWitness)
	if err != nil {
		return nil, fmt.Errorf("failed to generate Groth16 proof: %w", err)
	}

	// Serialize the proof to bytes
	var buf bytes.Buffer
	_, err = groth16Proof.WriteRawTo(&buf)
	if err != nil {
		return nil, fmt.Errorf("failed to serialize proof: %w", err)
	}

	return buf.Bytes(), nil
}

// VerifyWithTemplate verifies a proof against the public witness
func (p *Groth16Prover) Verify(proofBytes []byte, vk *types.Groth16VerifyingKey, publicWitness *types.TemplatePublicWitness) error {
	// Deserialize the proof from bytes
	proof := groth16.NewProof(p.curve.ToECC())
	buf := bytes.NewReader(proofBytes)
	_, err := proof.ReadFrom(buf)
	if err != nil {
		return fmt.Errorf("failed to deserialize proof: %w", err)
	}

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
	fmt.Printf("gnarkWitness: %#v\n", gnarkWitness)

	publicGnarkWitness, err := gnarkWitness.Public()
	if err != nil {
		return fmt.Errorf("failed to extract public witness: %w", err)
	}

	fmt.Printf("publicGnarkWitness: %#v\n", publicGnarkWitness)

	err = groth16.Verify(proof, vk.Key, publicGnarkWitness)
	if err != nil {
		return fmt.Errorf("Groth16 verification failed: %w", err)
	}

	return nil
}
