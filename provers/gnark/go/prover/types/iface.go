package types

type SerializableObject interface {
	Serialize() ([]byte, error)
	Deserialize([]byte, CurveType) error
}

type ToSolidityObject interface {
	ExportSolidity() ([]byte, error)
}
