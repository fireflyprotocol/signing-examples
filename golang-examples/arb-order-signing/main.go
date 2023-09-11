package main

import (
	"crypto/ecdsa"
	"fmt"
	"log"

	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/crypto"
)

// for each market there is a different trader contract
var TRADER_CONTRACT string = "0x934Dd6503795ef6EE6a36e3b3f1d7Be6c7096955"

// chain id for testnet its 421613, for mainnet its 42161
var NETWORK_ID string = "421613"

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

	hexPrivateKey := "0x2ee813034aab842141cb85d477f7d0e359838f46fcab34a935c69410a4d39efb"

	privateKey, err := crypto.HexToECDSA(hexPrivateKey[2:])
	if err != nil {
		log.Fatal(err)
	}

	pubKey := privateKey.Public()
	publicKeyECDSA, ok := pubKey.(*ecdsa.PublicKey)
	if !ok {
		log.Fatal("error casting public key to ECDSA")
	}

	walletAddress := crypto.PubkeyToAddress(*publicKeyECDSA)
	fmt.Println("Wallet Address: ", walletAddress, "\n")

	order := Order{
		is_buy:        true,
		reduce_only:   true,
		price:         toWeiStr(1800),
		quantity:      toWeiStr(6),
		leverage:      toWeiStr(0.02),
		trigger_price: toWeiStr(0),
		expiration:    "1690995498",
		salt:          "1231231231",
		maker:         walletAddress.String(),
	}

	fmt.Printf("Order: %+v\n\n", order)

	orderHash := getHash(order)
	fmt.Println("orderHash:", orderHash, "\n")

	orderSignature := signOrderHash(orderHash, privateKey)
	fmt.Println("orderSignature:", orderSignature, "\n")

	cancelOrderHash := getCancelHash(orderHash)
	fmt.Println("cancelOrderHash:", cancelOrderHash, "\n")

	cancelSignature := signOrderHash(cancelOrderHash, privateKey)
	fmt.Println("orderSignature:", cancelSignature, "\n")
}
