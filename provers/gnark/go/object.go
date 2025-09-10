package main

import (
	"log"
	"os"
	"sync"

	"github.com/tiannian/rsnark/provers-gnark/prover/types"
)

type ObjectCall struct{}

var (
	objects     []types.SerializableObject
	objectMutex sync.Mutex
)

func init() {
	ObjectImpl = ObjectCall{}
}

func (o ObjectCall) serialize(object_id *int64) []byte {
	objectMutex.Lock()
	defer objectMutex.Unlock()

	object := objects[*object_id]

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
	object := objects[*object_id]

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

	objectMutex.Lock()
	defer objectMutex.Unlock()
	objects = append(objects, object)

	return int64(len(objects) - 1)
}
