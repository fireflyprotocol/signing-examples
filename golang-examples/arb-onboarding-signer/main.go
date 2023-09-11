package main

import (
	"crypto/ecdsa"
	"fmt"
	"log"

	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/crypto"
)

func EVMSign(message string, privateKey *ecdsa.PrivateKey) (string, error) {
	fullMessage := fmt.Sprintf("\x19Ethereum Signed Message:\n%d%s", len(message), message)
	hash := crypto.Keccak256Hash([]byte(fullMessage))
	signatureBytes, err := crypto.Sign(hash.Bytes(), privateKey)
	if err != nil {
		return "", err
	}
	signatureBytes[64] += 27
	return hexutil.Encode(signatureBytes), nil
}

func main() {

	// for testnet this is the message to be signed, for mainnet use https://trade-arb.firefly.exchange
	msg := "https://testnet.firefly.exchange"

	// keccak256 hash of the data
	dataBytes := []byte(msg)
	hashData := crypto.Keccak256Hash(dataBytes)

	fmt.Println("message hash:", hashData, "\n")

	hexPrivateKey := "0x2ee813034aab842141cb85d477f7d0e359838f46fcab34a935c69410a4d39efb"

	privateKey, err := crypto.HexToECDSA(hexPrivateKey[2:])
	if err != nil {
		log.Fatal(err)
	}

	signature, err := EVMSign(string(hashData.Bytes()), privateKey)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("onboarding signature:", signature, "\n")
}
