package main

import (
	"log"
	"os"
	"sync"

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

func addObjectWithoutLock(obj types.SerializableObject) int64 {
	objectIDCounter++
	id := objectIDCounter
	objects[id] = obj

	return id
}

// addObject 添加一个新的object到map中，返回分配的ID
func addObject(obj types.SerializableObject) int64 {
	objectMutex.Lock()
	defer objectMutex.Unlock()

	return addObjectWithoutLock(obj)
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
		return deserializeObject[*types.Groth16CompiledCircuit](*curve_id, data)
	case 4:
		return deserializeObject[*types.PlonkProvingKey](*curve_id, data)
	case 5:
		return deserializeObject[*types.PlonkVerifyingKey](*curve_id, data)
	case 6:
		return deserializeObject[*types.PlonkCompiledCircuit](*curve_id, data)
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
		return readFromFile[*types.Groth16CompiledCircuit](*curve_id, path)
	case 4:
		return readFromFile[*types.PlonkProvingKey](*curve_id, path)
	case 5:
		return readFromFile[*types.PlonkVerifyingKey](*curve_id, path)
	case 6:
		return readFromFile[*types.PlonkCompiledCircuit](*curve_id, path)
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

func (o ObjectCall) remove_object(object_id *int64) {
	objectMutex.Lock()
	defer objectMutex.Unlock()

	delete(objects, *object_id)
}
