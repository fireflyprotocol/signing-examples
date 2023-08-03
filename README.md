# rust-examples
Repo containing rust code samples to interact with Bluefin protocol

### Arb Order Signing
In order to interact with our on-chain protocol, users must sign their orders off-chain before they can be posted to our orderbooks. The project shows how to generate EIP-712 signature for the order.

### Arb Onboarding Signer
Bluefin requires user to on-board onto exchange by signing a message off-chain using their wallet/account and sending it to exchange before they can start trading on our platform. The project provides an example on how to generate the onboarding signature.
