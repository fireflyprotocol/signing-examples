use ethers_signers::{LocalWallet, Signer};
use web3_unit_converter::Unit;

// module containing order struct and signing functionality
mod order;

#[tokio::main]
async fn main() {

    const WALLET_KEY: &str = "2ee813034aab842141cb85d477f7d0e359838f46fcab34a935c69410a4d39efb";

    // for each market there is a different trader contract
    const TRADER_CONTRACT: &str = "0x934Dd6503795ef6EE6a36e3b3f1d7Be6c7096955"; 

    // chain id for testnet its 421613, for mainnet its 42161
    const NETWORK_ID: &str = "421613";


    let wallet = WALLET_KEY.parse::<LocalWallet>().unwrap();
    let address = wallet.address();
    println!("Wallet address: {:?}\n", address);

    // create an order for signing
    let order = order::Order {
        is_buy: true,
        reduce_only: false,
        price: Unit::Ether(&"1601").to_wei_str().unwrap(), // in 1e18 format
        quantity: Unit::Ether(&"0.01").to_wei_str().unwrap(), // in 1e18 format
        leverage: Unit::Ether(&"20").to_wei_str().unwrap(), // in 1e18 format
        trigger_price:  Unit::Ether(&"0").to_wei_str().unwrap(), // in 1e18 format, always zero
        salt:"169332763775317".to_string(),
        expiration:"1696006037".to_string(),
        maker: address
    };

    println!("{:?}\n", order);

    let order_hash = order::get_hash(order, TRADER_CONTRACT, NETWORK_ID);
    println!("Order hash: 0x{}\n", order_hash);

    let signature =  order::sign_order(wallet, &order_hash).await;
    println!("signature: {}\n", signature);

    let order_hash_0x = "0x".to_string() + &order_hash;

    let cancel_order_hash = order::get_cancel_hash(&order_hash_0x, TRADER_CONTRACT, NETWORK_ID);
    println!("Cancel Order hash: 0x{}\n", cancel_order_hash);

    // sign cancellation hash
    let wallet = WALLET_KEY.parse::<LocalWallet>().unwrap();
    let cancel_signature =  order::sign_order(wallet, &cancel_order_hash).await;
    println!("Cancel signature: {}\n", cancel_signature);

}
