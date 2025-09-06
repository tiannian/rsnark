package main

import (
	"encoding/json"
	"fmt"
	"math/big"
	"strings"
)

type HexBigInt struct {
	*big.Int
}

func (h *HexBigInt) UnmarshalJSON(data []byte) error {
	str := strings.Trim(string(data), `"`)
	if strings.HasPrefix(str, "0x") || strings.HasPrefix(str, "0X") {
		n, ok := new(big.Int).SetString(str[2:], 16)
		if !ok {
			return fmt.Errorf("invalid hex number: %s", str)
		}
		h.Int = n
		return nil
	}
	// 默认按十进制
	n, ok := new(big.Int).SetString(str, 10)
	if !ok {
		return fmt.Errorf("invalid decimal number: %s", str)
	}
	h.Int = n
	return nil
}

type ParsedData struct {
	Value HexBigInt `json:"value"`
}

func main() {
	js1 := `{"value": "0x1f"}`

	var e1 ParsedData
	_ = json.Unmarshal([]byte(js1), &e1)

	fmt.Println(e1.Value.Text(10))
}
