package main

import (
	"crypto/ecdsa"
	"encoding/hex"
	"fmt"
	"log"
	"math/big"
	"strconv"

	"github.com/ethereum/go-ethereum/crypto"
	solsha3 "github.com/miguelmota/go-solidity-sha3"
	"github.com/shopspring/decimal"
)

var EIP712_ORDER_STRUCT_STRING string = "Order(bytes8 flags,uint128 quantity,uint128 price,uint128 triggerPrice,uint128 leverage,address maker,uint128 expiration)"
var EIP712_CANCEL_ORDER_STRUCT_STRING string = "CancelLimitOrder(string action,bytes32[] orderHashes)"
var EIP712_DOMAIN_STRING string = "EIP712Domain(string name,string version,uint128 chainId,address verifyingContract)"
var EIP712_DOMAIN_NAME string = "IsolatedTrader"
var EIP712_DOMAIN_VERSION string = "1.0"
var EIP712_PREFIX string = "1901"

type Order struct {
	is_buy        bool
	reduce_only   bool
	quantity      string
	price         string
	trigger_price string
	leverage      string
	expiration    string
	salt          string
	maker         string
}

/**
 * Helper method to convert to Wei -> return string format
 */

func toWeiStr(ether float64) string {
	amount := decimal.NewFromFloat(ether)
	mul := decimal.NewFromFloat(float64(10)).Pow(decimal.NewFromFloat(float64(18)))
	result := amount.Mul(mul)

	wei := new(big.Int)
	wei.SetString(result.String(), 10)

	return wei.String()
}

/**
 * Encodes order flags and returns a 16 bit hex
 */
func getOrderFlags(order Order) string {
	booleanFlag := 0

	if order.is_buy {
		booleanFlag += 1
	}

	if order.reduce_only {
		booleanFlag += 2
	}

	i, _ := strconv.ParseInt(order.salt, 10, 64)
	orderFlags := fmt.Sprintf("%015x%x", i, booleanFlag)

	return orderFlags
}

func addressToBytes32(address string) string {
	removed0x := address[2:]
	return "0x000000000000000000000000" + removed0x
}

func getOrderDataHash(order Order) string {
	orderFlags := getOrderFlags(order)

	orderFlagsDecoded, _ := hex.DecodeString(orderFlags)

	types := []string{"bytes32", "bytes32", "uint256", "uint256", "uint256", "uint256", "bytes32", "uint256"}
	values := []interface{}{
		crypto.Keccak256Hash([]byte(EIP712_ORDER_STRUCT_STRING)).Bytes(),
		orderFlagsDecoded,
		order.quantity,
		order.price,
		order.trigger_price,
		order.leverage,
		addressToBytes32(order.maker),
		order.expiration,
	}
	hash := solsha3.SoliditySHA3(types, values)

	return hex.EncodeToString(hash)
}

/**
 * Given an order hash, encodes its data and computes its keckak hash just like solidity
 */
func getOrderCancelHash(orderHash string) string {

	orderHashDecoded, _ := hex.DecodeString(orderHash[2:])
	hash := solsha3.SoliditySHA3(
		solsha3.Bytes32(orderHashDecoded),
	)

	types2 := []string{"bytes32", "bytes32", "bytes32"}
	values2 := []interface{}{
		crypto.Keccak256Hash([]byte(EIP712_CANCEL_ORDER_STRUCT_STRING)).Bytes(),
		crypto.Keccak256Hash([]byte("Cancel Orders")).Bytes(),
		hash,
	}

	hash2 := solsha3.SoliditySHA3(types2, values2)
	return hex.EncodeToString(hash2)
}

/**
 * Returns the EIP-712 style domain hash
 */
func getDomainSeparatorHash() string {
	types := []string{"bytes32", "bytes32", "bytes32", "uint256", "bytes32"}
	values := []interface{}{
		crypto.Keccak256Hash([]byte(EIP712_DOMAIN_STRING)).Bytes(),
		crypto.Keccak256Hash([]byte(EIP712_DOMAIN_NAME)).Bytes(),
		crypto.Keccak256Hash([]byte(EIP712_DOMAIN_VERSION)).Bytes(),
		NETWORK_ID,
		addressToBytes32(TRADER_CONTRACT),
	}
	hash := solsha3.SoliditySHA3(types, values)
	return hex.EncodeToString(hash)
}

func getEip712Hash(domainHash string, orderDataHash string) string {
	data := fmt.Sprintf("%s%s%s", EIP712_PREFIX, domainHash, orderDataHash)
	decodedData, _ := hex.DecodeString(data)
	msgHash := crypto.Keccak256Hash([]byte(decodedData))
	return msgHash.String()
}

/**
 * Given an order, trader contract address and network id,
 * returns EIP 712 hash of the order
 */
func getHash(order Order) string {

	orderDataHash := getOrderDataHash(order)
	domainHash := getDomainSeparatorHash()
	eip712OrderHash := getEip712Hash(domainHash, orderDataHash)

	return eip712OrderHash
}

/**
 * Given an order hash, trader contract address and network id,
 * returns EIP 712 cancel hash of the order
 */
func getCancelHash(orderHash string) string {
	orderCancellationHash := getOrderCancelHash(orderHash)
	domainHash := getDomainSeparatorHash()
	eip712CancelOrderHash := getEip712Hash(domainHash, orderCancellationHash)

	return eip712CancelOrderHash
}

func signOrderHash(orderHash string, privateKey *ecdsa.PrivateKey) string {
	decodedOrderHash, _ := hex.DecodeString(orderHash[2:])
	signature, err := EVMSign(string(decodedOrderHash), privateKey)
	if err != nil {
		log.Fatal(err)
	}
	return fmt.Sprintf("%s01", signature)
}
