package main

import (
	"sync"

	"github.com/tiannian/rsnark/provers-gnark/prover"
	prover_types "github.com/tiannian/rsnark/provers-gnark/prover/types"
)

type Groth16ProverCall struct{}

var (
	provers     []*prover.Groth16Prover
	proverMutex sync.Mutex
)

func init() {
	Groth16ProverImpl = Groth16ProverCall{}
}

func (p Groth16ProverCall) new(curve *uint64) uint64 {
	curveType := prover_types.CurveType(*curve)

	prover := prover.NewGroth16Prover(curveType)
	proverMutex.Lock()
	defer proverMutex.Unlock()

	provers = append(provers, prover)

	return uint64(len(provers) - 1)
}

func (p Groth16ProverCall) curve_id(prover_id *uint64) uint64 {
	proverMutex.Lock()
	defer proverMutex.Unlock()

	prover := provers[*prover_id]
	curve := prover.CurveId()

	return uint64(curve)
}

// func (p Groth16ProverCall) compile(prover_id *uint64, circuitData *[]byte) int64 {
// 	circuitDef, err := circuit.ParseCircuitDefinition(*circuitData)
// 	if err != nil {
// 		log.Fatalf("failed to parse circuit definition: %v", err)
// 		return -1
// 	}

// 	proverMutex.Lock()
// 	defer proverMutex.Unlock()

// 	prover := provers[*prover_id]
// 	compiled, err := prover.Compile(circuitDef)
// 	if err != nil {
// 		log.Fatalf("failed to compile circuit: %v", err)
// 		return -2
// 	}

// 	objectMutex.Lock()
// 	defer objectMutex.Unlock()

// 	objects = append(objects, compiled)

// 	return int64(len(objects) - 1)
// }
