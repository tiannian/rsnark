package main

import (
	"log"
	"sync"

	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/scs"
	"github.com/tiannian/rsnark/provers-gnark/circuit"
	"github.com/tiannian/rsnark/provers-gnark/prover"
	"github.com/tiannian/rsnark/provers-gnark/prover/types"
)

type PlonkProverCall struct{}

var (
	plonkProvers         map[uint64]*prover.PlonkProver
	plonkProverMutex     sync.Mutex
	plonkProverIDCounter uint64
)

func init() {
	PlonkProverImpl = PlonkProverCall{}
	plonkProvers = make(map[uint64]*prover.PlonkProver)
}

// addPlonkProver 添加一个新的PLONK prover到map中，返回分配的ID
func addPlonkProver(p *prover.PlonkProver) uint64 {
	plonkProverMutex.Lock()
	defer plonkProverMutex.Unlock()

	plonkProverIDCounter++
	id := plonkProverIDCounter
	plonkProvers[id] = p

	return id
}

func (p PlonkProverCall) plonk_create(curve *uint64) uint64 {
	curveType := types.CurveType(*curve)

	prover := prover.NewPlonkProver(curveType)
	return addPlonkProver(prover)
}

func (p PlonkProverCall) plonk_compile(curve_id *uint64, circuit_data *[]byte) int64 {
	curveType := types.CurveType(*curve_id)

	// Parse CircuitDefinition from JSON
	cd, err := circuit.ParseCircuitDefinition(*circuit_data)
	if err != nil {
		log.Fatalf("failed to parse circuit definition: %v", err)
		return -20013
	}

	// Create TemplateCircuit from CircuitDefinition
	templateCircuit, err := circuit.NewTemplateCircuit(cd)
	if err != nil {
		log.Fatalf("failed to create template circuit: %v", err)
		return -20014
	}

	scs, err := frontend.Compile(curveType.ToECC().ScalarField(), scs.NewBuilder, templateCircuit)
	if err != nil {
		log.Fatalf("failed to compile circuit to R1CS: %v", err)
		return -20015
	}

	compiled := &types.PlonkCompiledCircuit{
		CS: scs,
	}

	return addObject(compiled)
}

func (p PlonkProverCall) plonk_setup(prover_id *uint64, compiled_circuit_id *int64) []byte {
	plonkProverMutex.Lock()
	prover, proverExists := plonkProvers[*prover_id]
	plonkProverMutex.Unlock()
	if !proverExists {
		log.Fatalf("PLONK prover with id %d not found", *prover_id)
		return int64ToBytes2(-20011, 0)
	}

	objectMutex.Lock()
	compiledObj, objExists := objects[*compiled_circuit_id]
	objectMutex.Unlock()

	if !objExists {
		log.Fatalf("compiled circuit with id %d not found", *compiled_circuit_id)
		return int64ToBytes2(-20012, 0)
	}

	compiled, ok := compiledObj.(*types.PlonkCompiledCircuit)
	if !ok {
		log.Fatalf("failed to cast compiled circuit to types.PlonkCompiledCircuit")
		return int64ToBytes2(-20003, 0)
	}

	pk, vk, err := prover.Setup(compiled)

	if err != nil {
		log.Fatalf("failed to setup PLONK: %v", err)
		return int64ToBytes2(-20004, 0)
	}

	pkID := addObject(pk)
	vkID := addObject(vk)

	return int64ToBytes2(pkID, vkID)
}

func (p PlonkProverCall) plonk_prove(prover_id *uint64, compiled_circuit_id *int64, pk_id *int64, witness_data *[]byte) int64 {
	plonkProverMutex.Lock()
	prover, proverExists := plonkProvers[*prover_id]
	plonkProverMutex.Unlock()
	if !proverExists {
		log.Fatalf("PLONK prover with id %d not found", *prover_id)
		return -20011
	}

	objectMutex.Lock()
	defer objectMutex.Unlock()

	pkObj, pkExists := objects[*pk_id]
	if !pkExists {
		log.Fatalf("PLONK proving key with id %d not found", *pk_id)
		return -20012
	}

	pk, ok := pkObj.(*types.PlonkProvingKey)
	if !ok {
		log.Fatalf("failed to cast pk to types.PlonkProvingKey")
		return -20005
	}

	compiledObj, compiledExists := objects[*compiled_circuit_id]
	if !compiledExists {
		log.Fatalf("compiled circuit with id %d not found", *compiled_circuit_id)
		return -20012
	}

	compiled, ok := compiledObj.(*types.PlonkCompiledCircuit)
	if !ok {
		log.Fatalf("failed to cast compiled circuit to types.CompiledCircuit")
		return -20006
	}

	var witness types.TemplateWitness
	err := witness.FromJSON(*witness_data)
	if err != nil {
		log.Fatalf("failed to deserialize witness: %v", err)
		return -20007
	}

	proof, err := prover.Prove(compiled, pk, &witness)
	if err != nil {
		log.Fatalf("failed to prove: %v", err)
		return -20008
	}

	return addObjectWithoutLock(proof)
}

func (p PlonkProverCall) plonk_verify(prover_id *uint64, vk_id *int64, proof_id *int64, public_witness_data *[]byte) int64 {
	plonkProverMutex.Lock()
	prover, proverExists := plonkProvers[*prover_id]
	plonkProverMutex.Unlock()
	if !proverExists {
		log.Fatalf("PLONK prover with id %d not found", *prover_id)
		return -20011
	}

	objectMutex.Lock()
	defer objectMutex.Unlock()

	vkObj, vkExists := objects[*vk_id]
	if !vkExists {
		log.Fatalf("PLONK verifying key with id %d not found", *vk_id)
		return -20012
	}

	vk, ok := vkObj.(*types.PlonkVerifyingKey)
	if !ok {
		log.Fatalf("failed to cast vk to types.PlonkVerifyingKey")
		return -20009
	}

	var public_witness types.TemplatePublicWitness
	err := public_witness.FromJSON(*public_witness_data)
	if err != nil {
		log.Fatalf("failed to deserialize public witness: %v", err)
		return -20007
	}

	proofObj, proofExists := objects[*proof_id]
	if !proofExists {
		log.Fatalf("proof with id %d not found", *proof_id)
		return -20013
	}

	proof, ok := proofObj.(*types.PlonkProof)
	if !ok {
		log.Fatalf("failed to cast proof to types.PlonkProof")
		return -20014
	}

	err = prover.Verify(proof, vk, &public_witness)
	if err != nil {
		log.Fatalf("failed to verify: %v", err)
		return -20010
	}

	return int64(0)
}

func (p PlonkProverCall) plonk_remove_prover(prover_id *uint64) {
	plonkProverMutex.Lock()
	defer plonkProverMutex.Unlock()

	delete(plonkProvers, *prover_id)
}
