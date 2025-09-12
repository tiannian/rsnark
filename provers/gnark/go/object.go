package main

import (
	"log"
	"os"
	"sync"

	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"

	"github.com/tiannian/rsnark/provers-gnark/circuit"
	"github.com/tiannian/rsnark/provers-gnark/prover/types"
)

type ObjectCall struct{}

var (
	objects         map[int64]types.SerializableObject
	objectMutex     sync.Mutex
	objectIDCounter int64
)

func init() {
	ObjectImpl = ObjectCall{}
	objects = make(map[int64]types.SerializableObject)
}

// addObject 添加一个新的object到map中，返回分配的ID
func addObject(obj types.SerializableObject) int64 {
	objectMutex.Lock()
	defer objectMutex.Unlock()

	objectIDCounter++
	id := objectIDCounter
	objects[id] = obj

	return id
}

func (o ObjectCall) serialize(object_id *int64) []byte {
	objectMutex.Lock()
	defer objectMutex.Unlock()

	object, exists := objects[*object_id]
	if !exists {
		log.Fatalf("object with id %d not found", *object_id)
		return []byte{}
	}

	data, err := object.Serialize()
	if err != nil {
		log.Fatalf("failed to serialize object: %v", err)
		return []byte{}
	}

	return data
}

func (o ObjectCall) deserialize(ty *uint64, curve_id *uint64, data *[]byte) int64 {

	switch *ty {
	case 1:
		return deserializeObject[*types.Groth16ProvingKey](*curve_id, data)
	case 2:
		return deserializeObject[*types.Groth16VerifyingKey](*curve_id, data)
	case 3:
		return deserializeObject[*types.CompiledCircuit](*curve_id, data)
	}
	return 0
}

func (o ObjectCall) write_to_file(object_id *int64, path *string) int64 {
	objectMutex.Lock()
	defer objectMutex.Unlock()
	object, exists := objects[*object_id]
	if !exists {
		log.Fatalf("object with id %d not found", *object_id)
		return -20012
	}

	data, err := object.Serialize()
	if err != nil {
		log.Fatalf("failed to write object to file: %v", err)
		return -10003
	}

	err = os.WriteFile(*path, data, 0644)
	if err != nil {
		log.Fatalf("failed to write object to file: %v", err)
		return -10004
	}
	return 0
}

func (o ObjectCall) read_from_file(ty *uint64, curve_id *uint64, path *string) int64 {
	switch *ty {
	case 1:
		return readFromFile[*types.Groth16ProvingKey](*curve_id, path)
	case 2:
		return readFromFile[*types.Groth16VerifyingKey](*curve_id, path)
	case 3:
		return readFromFile[*types.CompiledCircuit](*curve_id, path)
	}

	return 0
}

func readFromFile[T types.SerializableObject](curve_id uint64, path *string) int64 {
	data, err := os.ReadFile(*path)
	if err != nil {
		log.Fatalf("failed to read file: %v", err)
		return -10002
	}

	return deserializeObject[T](curve_id, &data)
}

func deserializeObject[T types.SerializableObject](curve_id uint64, data *[]byte) int64 {
	curve := types.CurveType(curve_id)

	var object T
	err := object.Deserialize(*data, curve)
	if err != nil {
		log.Fatalf("failed to deserialize object: %v", err)
		return -10001
	}

	return addObject(object)
}

func (o ObjectCall) export_solidity(object_id *int64) []byte {
	objectMutex.Lock()
	defer objectMutex.Unlock()
	object, exists := objects[*object_id]
	if !exists {
		log.Fatalf("object with id %d not found", *object_id)
		return int64ToBytes(-20012)
	}

	pk, ok := object.(*types.Groth16VerifyingKey)
	if !ok {
		log.Fatalf("failed to cast object to types.Groth16VerifyingKey")
		return int64ToBytes(-10005)
	}

	solidity, err := pk.ExportSolidity()
	if err != nil {
		log.Fatalf("failed to export solidity: %v", err)
		return int64ToBytes(-10006)
	}

	return append(int64ToBytes(0), solidity...)
}

func (o ObjectCall) compile(curve_id *uint64, circuit_data *[]byte) int64 {
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

	// Compile to R1CS
	r1cs, err := frontend.Compile(curveType.ToECC().ScalarField(), r1cs.NewBuilder, templateCircuit)
	if err != nil {
		log.Fatalf("failed to compile circuit to R1CS: %v", err)
		return -20015
	}

	compiled := &types.CompiledCircuit{
		CS: r1cs,
	}

	return addObject(compiled)
}

func (o ObjectCall) remove_object(object_id *int64) {
	objectMutex.Lock()
	defer objectMutex.Unlock()

	delete(objects, *object_id)
}
