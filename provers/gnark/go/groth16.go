package main

import (
	"encoding/binary"
	"log"
	"sync"

	"github.com/tiannian/rsnark/provers-gnark/circuit"
	"github.com/tiannian/rsnark/provers-gnark/prover"
	"github.com/tiannian/rsnark/provers-gnark/prover/types"
)

type Groth16ProverCall struct{}

var (
	provers     []*prover.Groth16Prover
	proverMutex sync.Mutex
)

func init() {
	Groth16ProverImpl = Groth16ProverCall{}
}

func (p Groth16ProverCall) create(curve *uint64) uint64 {
	curveType := types.CurveType(*curve)

	prover := prover.NewGroth16Prover(curveType)
	proverMutex.Lock()
	defer proverMutex.Unlock()

	provers = append(provers, prover)

	return uint64(len(provers) - 1)
}

func (p Groth16ProverCall) compile(prover_id *uint64, circuitData *[]byte) int64 {
	circuitDef, err := circuit.ParseCircuitDefinition(*circuitData)
	if err != nil {
		log.Fatalf("failed to parse circuit definition: %v", err)
		return -20001
	}

	proverMutex.Lock()
	defer proverMutex.Unlock()

	prover := provers[*prover_id]
	compiled, err := prover.Compile(circuitDef)
	if err != nil {
		log.Fatalf("failed to compile circuit: %v", err)
		return -20002
	}

	objectMutex.Lock()
	defer objectMutex.Unlock()

	objects = append(objects, compiled)

	return int64(len(objects) - 1)
}

func (p Groth16ProverCall) setup(prover_id *uint64, compiled_circuit_id *int64) []byte {
	proverMutex.Lock()
	defer proverMutex.Unlock()

	objectMutex.Lock()
	defer objectMutex.Unlock()

	prover := provers[*prover_id]
	compiled, ok := objects[*compiled_circuit_id].(*types.CompiledCircuit)
	if !ok {
		log.Fatalf("failed to cast compiled circuit to types.CompiledCircuit")
		return int64ToBytes2(-20003, 0)
	}

	pk, vk, err := prover.Setup(compiled)

	if err != nil {
		log.Fatalf("failed to setup Groth16: %v", err)
		return int64ToBytes2(-20004, 0)
	}

	objects = append(objects, pk)
	res0 := int64(len(objects) - 1)

	objects = append(objects, vk)
	res1 := int64(len(objects) - 1)

	return int64ToBytes2(res0, res1)
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
	defer proverMutex.Unlock()

	objectMutex.Lock()
	defer objectMutex.Unlock()

	prover := provers[*prover_id]
	pk, ok := objects[*pk_id].(*types.Groth16ProvingKey)
	if !ok {
		log.Fatalf("failed to cast pk to types.Groth16ProvingKey")
		return int64ToBytes(-20005)
	}

	compiled, ok := objects[*compiled_circuit_id].(*types.CompiledCircuit)
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
	defer proverMutex.Unlock()

	objectMutex.Lock()
	defer objectMutex.Unlock()

	prover := provers[*prover_id]
	vk, ok := objects[*vk_id].(*types.Groth16VerifyingKey)
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
