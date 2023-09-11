# signing-examples
Repo containing rust and golang code samples to interact with the Bluefin protocol

### Arb Onboarding Signer
Bluefin requires users to onboard onto the exchange by signing a message off-chain using their wallet/account and sending it to exchange before they can start trading on our platform. The project provides an example on how to generate the onboarding signature.

### Arb Order Signing
In order to interact with our on-chain protocol, users must sign their orders off-chain before they can be posted to our orderbooks. To cancel an order, a user must sign a cancellation hash based on the order hash.

The project shows how to generate an EIP-712 signature and its corresponding cancellation signature for a particular order

## Running Examples

### Golang
1. Change directory to specific project eg: ```cd golang-examples\arb-onboarding-signer```
2. ```go run .```

### Rust
1. Change directory to specific project eg: ```cd rust-examples\arb-onboarding-signer```
2. ```cargo run```