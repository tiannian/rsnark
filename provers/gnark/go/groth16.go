package main

import (
	"encoding/binary"
	"log"
	"sync"

	"github.com/tiannian/rsnark/provers-gnark/prover"
	"github.com/tiannian/rsnark/provers-gnark/prover/types"
)

type Groth16ProverCall struct{}

var (
	provers         map[uint64]*prover.Groth16Prover
	proverMutex     sync.Mutex
	proverIDCounter uint64
)

func init() {
	Groth16ProverImpl = Groth16ProverCall{}
	provers = make(map[uint64]*prover.Groth16Prover)
}

// addProver 添加一个新的prover到map中，返回分配的ID
func addProver(p *prover.Groth16Prover) uint64 {
	proverMutex.Lock()
	defer proverMutex.Unlock()

	proverIDCounter++
	id := proverIDCounter
	provers[id] = p

	return id
}

func (p Groth16ProverCall) create(curve *uint64) uint64 {
	curveType := types.CurveType(*curve)

	prover := prover.NewGroth16Prover(curveType)
	return addProver(prover)
}

func (p Groth16ProverCall) setup(prover_id *uint64, compiled_circuit_id *int64) []byte {
	proverMutex.Lock()
	prover, proverExists := provers[*prover_id]
	proverMutex.Unlock()
	if !proverExists {
		log.Fatalf("prover with id %d not found", *prover_id)
		return int64ToBytes2(-20011, 0)
	}

	objectMutex.Lock()
	compiledObj, objExists := objects[*compiled_circuit_id]
	objectMutex.Unlock()

	if !objExists {
		log.Fatalf("compiled circuit with id %d not found", *compiled_circuit_id)
		return int64ToBytes2(-20012, 0)
	}

	compiled, ok := compiledObj.(*types.CompiledCircuit)
	if !ok {
		log.Fatalf("failed to cast compiled circuit to types.CompiledCircuit")
		return int64ToBytes2(-20003, 0)
	}

	pk, vk, err := prover.Setup(compiled)

	if err != nil {
		log.Fatalf("failed to setup Groth16: %v", err)
		return int64ToBytes2(-20004, 0)
	}

	pkID := addObject(pk)
	vkID := addObject(vk)

	return int64ToBytes2(pkID, vkID)
}

func int64ToBytes2(i0 int64, i1 int64) []byte {
	bigEndian := make([]byte, 16)
	binary.BigEndian.PutUint64(bigEndian, uint64(i0))
	binary.BigEndian.PutUint64(bigEndian[8:], uint64(i1))

	return bigEndian
}

func int64ToBytes(i0 int64) []byte {
	bigEndian := make([]byte, 8)
	binary.BigEndian.PutUint64(bigEndian, uint64(i0))

	return bigEndian
}

func (p Groth16ProverCall) prove(prover_id *uint64, compiled_circuit_id *int64, pk_id *int64, witness_data *[]byte) []byte {
	proverMutex.Lock()
	prover, proverExists := provers[*prover_id]
	proverMutex.Unlock()
	if !proverExists {
		log.Fatalf("prover with id %d not found", *prover_id)
		return int64ToBytes(-20011)
	}

	objectMutex.Lock()
	defer objectMutex.Unlock()

	pkObj, pkExists := objects[*pk_id]
	if !pkExists {
		log.Fatalf("proving key with id %d not found", *pk_id)
		return int64ToBytes(-20012)
	}

	pk, ok := pkObj.(*types.Groth16ProvingKey)
	if !ok {
		log.Fatalf("failed to cast pk to types.Groth16ProvingKey")
		return int64ToBytes(-20005)
	}

	compiledObj, compiledExists := objects[*compiled_circuit_id]
	if !compiledExists {
		log.Fatalf("compiled circuit with id %d not found", *compiled_circuit_id)
		return int64ToBytes(-20012)
	}

	compiled, ok := compiledObj.(*types.CompiledCircuit)
	if !ok {
		log.Fatalf("failed to cast compiled circuit to types.CompiledCircuit")
		return int64ToBytes(-20006)
	}

	var witness types.TemplateWitness
	err := witness.FromJSON(*witness_data)
	if err != nil {
		log.Fatalf("failed to deserialize witness: %v", err)
		return int64ToBytes(-20007)
	}

	proof, err := prover.Prove(compiled, pk, &witness)
	if err != nil {
		log.Fatalf("failed to prove: %v", err)
		return int64ToBytes(-20008)
	}

	code := int64ToBytes(0)
	return append(code, proof...)

}

func (p Groth16ProverCall) verify(prover_id *uint64, vk_id *int64, proof_data *[]byte, public_witness_data *[]byte) int64 {
	proverMutex.Lock()
	prover, proverExists := provers[*prover_id]
	proverMutex.Unlock()
	if !proverExists {
		log.Fatalf("prover with id %d not found", *prover_id)
		return -20011
	}

	objectMutex.Lock()
	defer objectMutex.Unlock()

	vkObj, vkExists := objects[*vk_id]
	if !vkExists {
		log.Fatalf("verifying key with id %d not found", *vk_id)
		return -20012
	}

	vk, ok := vkObj.(*types.Groth16VerifyingKey)
	if !ok {
		log.Fatalf("failed to cast vk to types.Groth16VerifyingKey")
		return -20009
	}

	var public_witness types.TemplatePublicWitness
	err := public_witness.FromJSON(*public_witness_data)
	if err != nil {
		log.Fatalf("failed to deserialize public witness: %v", err)
		return -20007
	}

	err = prover.Verify(*proof_data, vk, &public_witness)
	if err != nil {
		log.Fatalf("failed to verify: %v", err)
		return -20010
	}

	return int64(0)
}

func (p Groth16ProverCall) compile(curve_id *uint64, circuit_data *[]byte) int64 {
	// Delegate to object's compile function
	return ObjectImpl.compile(curve_id, circuit_data)
}

func (p Groth16ProverCall) remove_prover(prover_id *uint64) {
	proverMutex.Lock()
	defer proverMutex.Unlock()

	delete(provers, *prover_id)
}
